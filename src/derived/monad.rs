use crate::{associative_flatten::*, covariant::*, hkt::*};

pub trait Monad: AssociativeFlatten + Covariant {
    fn flat_map<
        A: 'static,
        B: 'static,
        F: FnMut(A) -> Self::Member<B> + 'static,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map(fa, f))
    }
}
pub trait MonadClone: Monad + CovariantClone {
    fn flat_map_<
        A: 'static,
        B: 'static,
        F: FnMut(&A) -> Self::Member<B> + 'static,
    >(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map_(fa, f))
    }
}
impl<T: AssociativeFlatten + Covariant> Monad for T {}

pub trait MonadSyntax<Mon: Monad, A: 'static>:
    Mirror<T = A, Family = Mon>
{
    fn flat_map<B: 'static, F: FnMut(Self::T) -> Mon::Member<B> + 'static>(
        self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map::<Self::T, B, F>(self.as_member(), f)
    }
}
impl<F: Monad, A: 'static, T: Mirror<Family = F, T = A>> MonadSyntax<F, A>
    for T
{
}

pub trait MonadCloneSyntax<Mon: MonadClone, A: 'static>:
    Mirror<Family = Mon, T = A>
{
    fn flat_map_<B: 'static, F: FnMut(&Self::T) -> Mon::Member<B> + 'static>(
        &self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map_(self.as_member_(), f)
    }
}
impl<F: MonadClone, A: 'static, T: Mirror<Family = F, T = A>>
    MonadCloneSyntax<F, A> for T
{
}
