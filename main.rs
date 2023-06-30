use std::{marker::PhantomData, rc::Rc};

use ratz::{syntax::*, *};

pub struct Thunk<'a, A, B>(Box<dyn FnOnce(A) -> B + 'a>);

impl<'a, A, B> Thunk<'a, A, B> {
  pub fn new<F>(f: F) -> Self
  where
    F: FnOnce(A) -> B + 'a,
  {
    Self(Box::new(f))
  }

  pub fn call(self, a: A) -> B {
    (self.0)(a)
  }
}
pub struct State<S, T>(Thunk<'static, S, (S, T)>);

impl<S, A> Mirror1 for State<S, A> {
  type A = A;
  type Constructor = StateFamily<S>;
}

pub struct StateFamily<S>(PhantomData<S>);

impl<S> TypeConstructor1 for StateFamily<S> {
  type K<A> = State<S, A>;
}

impl<S: 'static> Functor for StateFamily<S> {
  fn map<A: 'static, B, F: Fn(A) -> B + 'static>(
    fa: Self::K<A>,
    f: F,
  ) -> Self::K<B> {
    let f = Box::new(f);
    State(Thunk::new(move |state| {
      let (state, a) = fa.0.call(state);
      (state, f(a))
    }))
  }
}

impl<S: 'static> Applicative for StateFamily<S> {
  fn pure<A: 'static>(a: A) -> Self::K<A> {
    State(Thunk::new(move |state| (state, a)))
  }

  fn zip<A: 'static, B: 'static>(
    fa: Self::K<A>,
    fb: Self::K<B>,
  ) -> Self::K<(A, B)> {
    State(Thunk::new(move |state| {
      let (state, a) = fa.0.call(state);
      let (state, b) = fb.0.call(state);
      (state, (a, b))
    }))
  }
}

impl<S: 'static> Monad for StateFamily<S> {
  fn flat_map<A: 'static, B: 'static, F: Fn(A) -> Self::K<B> + 'static>(
    fa: Self::K<A>,
    f: F,
  ) -> Self::K<B> {
    State(Thunk::new(move |state| {
      let (state, a) = fa.0.call(state);
      f(a).0.call(state)
    }))
  }
}

fn run_state<S, A>(state: S, action: State<S, A>) -> (S, A) {
  action.0.call(state)
}

fn get<S: Clone>() -> State<S, S> {
  State(Thunk::new(move |state: S| (state.clone(), state)))
}

fn put<S: 'static>(state: S) -> State<S, ()> {
  State(Thunk::new(move |_| (state, ())))
}

#[macro_export]
macro_rules! mdo {
    ($ident:ident <- $expr:expr; $($tt:tt)*) => {
        $expr.flat_map(move |$ident| {
            mdo!($($tt)*)
        })
    };
    ($ident:ident = $expr:expr; $($tt:tt)*) => {
        {
            let $ident = $expr;
            mdo!($($tt)*)
        }
    };
    (_ = $expr:expr; $($tt:tt)*) => {
        mdo!(_expr = $expr; $($tt)*)
    };
    ($expr:expr; $($tt:tt)*) => {
        mdo!(_ident <- $expr; $($tt)*)
    };
    ($expr:expr) => {$expr};
}
fn main() {
  println!(
    "{:?}",
    run_state(
      10,
      mdo! {
          state <- get();
          put(state+10);
          get()
      }
    )
  );

  use example::*;
  run_bank_account_m(
    BankAccount {
      name: "abby".to_string(),
      balance: 1000,
    },
    atm(),
  );
}

pub mod example {
  use std::cell::RefCell;

  use super::*;

  pub struct BankAccount {
    pub name: String,
    pub balance: usize,
  }

  #[derive(Clone)]
  pub struct BankAccountRef(Rc<RefCell<BankAccount>>);

  impl BankAccountRef {
    pub fn new(acct: BankAccount) -> Self {
      Self(Rc::new(RefCell::new(acct)))
    }
  }

  pub type BankAccountM<T> = State<BankAccountRef, T>;

  pub fn get_account() -> BankAccountM<BankAccountRef> {
    get()
  }

  pub fn get_name() -> BankAccountM<String> {
    get_account().map(|acct| acct.0.borrow().name.clone())
  }

  pub fn get_balance() -> BankAccountM<usize> {
    get_account().map(|acct| acct.0.borrow().balance)
  }

  pub fn update_balance<F: Fn(&mut usize) + 'static>(f: F) -> BankAccountM<()> {
    mdo! {
        acct <- get_account();
        _ = f(&mut acct.0.borrow_mut().balance);
        StateFamily::pure(())
    }
  }

  pub fn run_bank_account_m<T>(acct: BankAccount, action: BankAccountM<T>) {
    run_state(BankAccountRef::new(acct), action);
  }

  pub fn atm() -> BankAccountM<()> {
    mdo! {
        name <- get_name();
        balance <- get_balance();
        _ = println!("hello {name}, you have {balance}");

        update_balance(|b|*b+=100);

        balance <- get_balance();
        _ = println!("new balance: {balance}");

        StateFamily::pure(())
    }
  }
}
