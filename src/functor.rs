use super::hkt::*;

pub trait Functor: TypeConstructor1 {
  fn map<A, B, F: Fn(A) -> B>(fa: Self::K<A>, f: F) -> Self::K<B>;
}

pub trait FunctorSyntax<TC: Functor, A>:
  Mirror1<Constructor = TC, A = A>
{
  fn map<B, F: Fn(A) -> B>(self, f: F) -> TC::K<B> {
    TC::map(self.reify(), f)
  }
}

impl<F: Functor, A, FA: Mirror1<Constructor = F, A = A>> FunctorSyntax<F, A>
  for FA
{
}
