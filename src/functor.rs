use super::hkt::*;

pub trait Functor: TypeConstructor1 {
  fn map<A: 'static, B, F: Fn(A) -> B + 'static>(
    fa: Self::K<A>,
    f: F,
  ) -> Self::K<B>;
}

pub trait FunctorSyntax<TC: Functor, A: 'static>:
  Mirror1<Constructor = TC, A = A>
{
  fn map<B, F: Fn(A) -> B + 'static>(self, f: F) -> TC::K<B> {
    TC::map(self.reify(), f)
  }
}

impl<F: Functor, A: 'static, FA: Mirror1<Constructor = F, A = A>>
  FunctorSyntax<F, A> for FA
{
}

impl<F: Functor, A: 'static, FA: Mirror1<Constructor = F, A = A>> Functor for FA {}
