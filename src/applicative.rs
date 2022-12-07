use super::hkt::*;
use super::Functor;

pub trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::Of<A>;
    fn zip<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<(A, B)>;
}

pub trait ApplicativeSyntax<TC: Applicative, A>: Mirror1<Constructor = TC, A = A> {
    fn zip<B: Clone>(self, fb: TC::Of<B>) -> TC::Of<(A, B)> {
        TC::zip(self.as_member(), fb)
    }
}

impl<F: Applicative, A, FA: Mirror1<Constructor = F, A = A>> ApplicativeSyntax<F, A> for FA {}
