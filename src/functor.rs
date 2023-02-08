use super::hkt::*;

pub trait Functor: TypeConstructor1 {
  fn map<A, B, F: Fn(A) -> B>(fa: Self::Of<A>, f: F) -> Self::Of<B>;
}

pub trait FunctorSyntax<TC: Functor, A>:
  Mirror1<Constructor = TC, A = A>
{
  fn map<B, F: Fn(A) -> B>(self, f: F) -> TC::Of<B> {
    TC::map(self.as_member(), f)
  }
}

impl<F: Functor, A, FA: Mirror1<Constructor = F, A = A>> FunctorSyntax<F, A>
  for FA
{
}
