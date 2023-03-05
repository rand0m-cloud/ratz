use super::hkt::*;
use super::Applicative;

pub trait Monad: Applicative {
  fn flat_map<A, B, F: Fn(A) -> Self::K<B>>(fa: Self::K<A>, f: F)
    -> Self::K<B>;
}

pub trait MonadSyntax<TC: Monad, A>: Mirror1<Constructor = TC, A = A> {
  fn flat_map<B, F: Fn(A) -> TC::K<B>>(self, f: F) -> TC::K<B> {
    TC::flat_map(self.reify(), f)
  }
}

impl<F: Monad, A, FA: Mirror1<Constructor = F, A = A>> MonadSyntax<F, A>
  for FA
{
}
