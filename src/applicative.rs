use super::hkt::*;
use super::Functor;

pub trait Applicative: Functor {
  fn pure<A>(a: A) -> Self::K<A>;
  fn zip<A, B>(fa: Self::K<A>, fb: Self::K<B>) -> Self::K<(A, B)>;
}

pub trait ApplicativeSyntax<TC: Applicative, A>:
  Mirror1<Constructor = TC, A = A>
{
  fn zip<B>(self, fb: TC::K<B>) -> TC::K<(A, B)> {
    TC::zip(self.reify(), fb)
  }
}

impl<F: Applicative, A, FA: Mirror1<Constructor = F, A = A>>
  ApplicativeSyntax<F, A> for FA
{
}
