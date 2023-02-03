use super::{Functor, Foldable, Applicative};
use super::hkt::Mirror1;

pub trait Traversable: Functor + Foldable {
    fn traverse<App: Applicative, A, B, F: Fn(A) -> App::Of<B>>(fa: Self::Of<A>, f: F) -> App::Of<Self::Of<B>>;
}

pub trait TraversableSyntax<TC: Traversable, A>: Mirror1<Constructor = TC, A = A> {
    fn traverse<App: Applicative, B, F: Fn(A) -> App::Of<B>>(self, f: F) -> App::Of<TC::Of<B>> {
        TC::traverse::<App, A, B, F>(self.as_member(), f)
    }
}

impl<F: Traversable, A, FA: Mirror1<Constructor = F, A = A>> TraversableSyntax<F, A> for FA {}
