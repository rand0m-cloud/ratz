use super::hkt::*;
use super::Applicative;

pub trait Monad: Applicative {
  fn flat_map<A: 'static, B: 'static, F: Fn(A) -> Self::K<B> + 'static>(
    fa: Self::K<A>,
    f: F,
  ) -> Self::K<B>;
}

pub trait MonadSyntax<TC: Monad, A: 'static>:
  Mirror1<Constructor = TC, A = A>
{
  fn flat_map<B: 'static, F: Fn(A) -> TC::K<B> + 'static>(
    self,
    f: F,
  ) -> TC::K<B> {
    TC::flat_map(self.reify(), f)
  }
}

impl<F: Monad, A: 'static, FA: Mirror1<Constructor = F, A = A>>
  MonadSyntax<F, A> for FA
{
}
