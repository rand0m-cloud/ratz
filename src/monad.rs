use super::hkt::*;
use super::Applicative;

pub trait Monad: Applicative {
    fn flat_map<A, B, F: Fn(A) -> Self::Of<B>>(fa: Self::Of<A>, f: F) -> Self::Of<B>;
}

pub trait MonadSyntax<TC: Monad, A>: Mirror1<Constructor = TC, A = A> {
    fn flat_map<B, F: Fn(A) -> TC::Of<B>>(self, f: F) -> TC::Of<B> {
        TC::flat_map(self.as_member(), f)
    }
}

impl<F: Monad, A, FA: Mirror1<Constructor = F, A = A>> MonadSyntax<F, A> for FA {}
