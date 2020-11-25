use crate::{associative_flatten::*, covariant::*, hkt::*};

pub trait Monad: AssociativeFlatten + Covariant {
    fn flat_map<A, B, F: FnMut(A) -> Self::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map(fa, f))
    }
}
pub trait MonadClone: Monad + CovariantClone {
    fn flat_map_<A, B, F: FnMut(&A) -> Self::Member<B>>(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map_(fa, f))
    }
}
impl<T: AssociativeFlatten + Covariant> Monad for T {}

pub trait MonadSyntax<Mon: Monad, A>: Mirror<T = A, Family = Mon> {
    fn flat_map<B, F: FnMut(Self::T) -> Mon::Member<B>>(
        self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map::<Self::T, B, F>(self.as_member(), f)
    }
}
impl<F: Monad, A, T: Mirror<Family = F, T = A>> MonadSyntax<F, A> for T {}

pub trait MonadCloneSyntax<Mon: MonadClone, A>:
    Mirror<Family = Mon, T = A>
{
    fn flat_map_<B, F: FnMut(&Self::T) -> Mon::Member<B>>(
        &self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map_(self.as_member_(), f)
    }
}
impl<F: MonadClone, A, T: Mirror<Family = F, T = A>> MonadCloneSyntax<F, A>
    for T
{
}
