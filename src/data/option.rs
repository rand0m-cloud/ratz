use functor::FunctorSyntax;

use crate::hkt::{Mirror1, TypeConstructor1};
use crate::*;

impl<A> Mirror1 for Option<A> {
  type A = A;
  type Constructor = OptionFamily;
}

pub struct OptionFamily;

impl TypeConstructor1 for OptionFamily {
  type K<A> = Option<A>;
}

impl Functor for OptionFamily {
  fn map<A, B, F: Fn(A) -> B>(fa: Option<A>, f: F) -> Option<B> {
    fa.map(f)
  }
}

impl Applicative for OptionFamily {
  fn pure<A>(a: A) -> Option<A> {
    Some(a)
  }

  fn zip<A, B>(fa: Option<A>, fb: Option<B>) -> Option<(A, B)> {
    fa.zip(fb)
  }
}

impl Monad for OptionFamily {
  fn flat_map<A, B, F: Fn(A) -> Option<B>>(fa: Option<A>, f: F) -> Option<B> {
    fa.and_then(f)
  }
}

impl Foldable for OptionFamily {
  fn fold<A, S, F: Fn(S, A) -> S>(fa: Option<A>, initial: S, f: F) -> S {
    match fa {
      None => initial,
      Some(a) => f(initial, a),
    }
  }
}

impl Traversable for OptionFamily {
  fn traverse<App: Applicative, A, B, F: Fn(A) -> App::K<B> + 'static>(
    fa: Option<A>,
    f: F,
  ) -> App::K<Option<B>> {
    match fa {
      None => App::pure(None),
      Some(a) => f(a).map(Some),
    }
  }
}
