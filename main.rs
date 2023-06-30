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

impl<S: 'static, A: 'static> Mirror1 for State<S, A> {
  type A = A;
  type Constructor = StateFamily<S>;
}

pub struct StateFamily<S>(PhantomData<S>);

impl<S: 'static> TypeConstructor1 for StateFamily<S> {
  type K<A: 'static> = State<S, A>;
}

impl<S: 'static> Functor for StateFamily<S> {
  fn map<A: 'static, B: 'static, F: Fn(A) -> B + 'static>(
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
  test_transformer();
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

pub struct EnvT<M: TypeConstructor1, E, A: 'static>(
  Thunk<'static, E, M::K<A>>,
  PhantomData<M>,
);

pub struct EnvTFamily<M, E>(PhantomData<(M, E)>);

impl<M: TypeConstructor1 + 'static, E: 'static> TypeConstructor1
  for EnvTFamily<M, E>
{
  type K<A: 'static> = EnvT<M, E, A>;
}

impl<M: TypeConstructor1 + 'static, E: 'static, A: 'static> Mirror1
  for EnvT<M, E, A>
{
  type A = A;
  type Constructor = EnvTFamily<M, E>;
}

impl<M: TypeConstructor1 + Functor + 'static, E: 'static> Functor
  for EnvTFamily<M, E>
{
  fn map<A: 'static, B: 'static, F: Fn(A) -> B + 'static>(
    fa: EnvT<M, E, A>,
    f: F,
  ) -> EnvT<M, E, B> {
    EnvT(
      Thunk::new(move |env| {
        let a = fa.0.call(env);
        a.map(f)
      }),
      PhantomData,
    )
  }
}

impl<M: TypeConstructor1 + Functor + Applicative + 'static, E: 'static>
  Applicative for EnvTFamily<M, E>
{
  fn pure<A: 'static>(a: A) -> EnvT<M, E, A> {
    EnvT(Thunk::new(move |_| M::pure(a)), PhantomData)
  }

  fn zip<A: 'static, B: 'static>(
    fa: Self::K<A>,
    fb: Self::K<B>,
  ) -> Self::K<(A, B)> {
    EnvT(Thunk::new(move |env| fa.zip(fb).0.call(env)), PhantomData)
  }
}

impl<
    M: TypeConstructor1 + Functor + Applicative + Monad + 'static,
    E: 'static + Clone,
  > Monad for EnvTFamily<M, E>
{
  fn flat_map<A: 'static, B: 'static, F: Fn(A) -> Self::K<B> + 'static>(
    fa: Self::K<A>,
    f: F,
  ) -> Self::K<B> {
    EnvT(
      Thunk::new(move |env: E| {
        fa.0
          .call(env.clone())
          .flat_map(move |a| f(a).0.call(env.clone()))
      }),
      PhantomData,
    )
  }
}

trait MonadTrans<M: TypeConstructor1>: TypeConstructor1 + Monad {
  fn lift<A: 'static>(base: M::K<A>) -> Self::K<A>;
}

impl<M: TypeConstructor1 + Monad + 'static, E: 'static + Clone> MonadTrans<M>
  for EnvTFamily<M, E>
{
  fn lift<A: 'static>(base: M::K<A>) -> EnvT<M, E, A> {
    EnvT(Thunk::new(move |_| base), PhantomData)
  }
}

fn get_env<M: TypeConstructor1 + Monad, S>() -> EnvT<M, S, S> {
  EnvT(Thunk::new(|state| M::pure(state)), PhantomData)
}

fn run_env_t<M: TypeConstructor1 + Monad, S, A>(
  env: S,
  m: EnvT<M, S, A>,
) -> M::K<A> {
  m.0.call(env)
}

fn test_transformer() {
  let action: EnvT<StateFamily<String>, usize, _> = mdo! {
      state <- EnvTFamily::lift(get());
      env <- get_env();
      _ = println!("got state: {state} and env: {env}");
      EnvTFamily::pure(())
  };
  run_state("test state".to_string(), run_env_t(10000, action));
}
