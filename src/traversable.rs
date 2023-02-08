use super::hkt::Mirror1;
use super::{Applicative, Foldable, Functor};

pub trait Traversable: Functor + Foldable {
  fn traverse<App: Applicative, A, B, F: Fn(A) -> App::K<B>>(
    fa: Self::K<A>,
    f: F,
  ) -> App::K<Self::K<B>>;
}

pub trait TraversableSyntax<TC: Traversable, A>:
  Mirror1<Constructor = TC, A = A>
{
  fn traverse<App: Applicative, B, F: Fn(A) -> App::K<B>>(
    self,
    f: F,
  ) -> App::K<TC::K<B>> {
    TC::traverse::<App, A, B, F>(self.reify(), f)
  }
}

impl<F: Traversable, A, FA: Mirror1<Constructor = F, A = A>>
  TraversableSyntax<F, A> for FA
{
}
