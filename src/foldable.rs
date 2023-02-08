use super::hkt::{Mirror1, TypeConstructor1};
use super::Monoid;

pub trait Foldable: TypeConstructor1 {
  fn fold<A, S, F: Fn(S, A) -> S>(fa: Self::K<A>, initial: S, f: F) -> S;
}

pub trait FoldableSyntax<TC: Foldable, A>:
  Mirror1<Constructor = TC, A = A>
{
  fn fold<S, F: Fn(S, A) -> S>(self, initial: S, f: F) -> S {
    TC::fold(self.reify(), initial, f)
  }

  fn fold_map<B: Monoid, F: Fn(A) -> B>(self, f: F) -> B {
    self.fold(Monoid::empty(), |s, a| s.combine(f(a)))
  }
}

impl<F: Foldable, A, FA: Mirror1<Constructor = F, A = A>> FoldableSyntax<F, A>
  for FA
{
}
