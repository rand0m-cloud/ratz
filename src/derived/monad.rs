use crate::{associative_flatten::*, covariant::*, hkt::*};

pub trait Monad<'a>: AssociativeFlatten<'a> + Covariant<'a> {
    fn flat_map<A: 'a, B: 'a, F: FnMut(A) -> Self::Member<B> + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map(fa, f))
    }
}
pub trait MonadClone<'a>: Monad<'a> + CovariantClone<'a> {
    fn flat_map_<A: 'a, B: 'a, F: FnMut(&A) -> Self::Member<B> + 'a>(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map_(fa, f))
    }
}
impl<'a, T: AssociativeFlatten<'a> + Covariant<'a>> Monad<'a> for T {}

pub trait MonadSyntax<'a, Mon: Monad<'a>>: Mirror<'a, Family = Mon> {
    fn flat_map<B: 'a, F: FnMut(Self::T) -> Mon::Member<B> + 'a>(
        self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map(self.as_member(), f)
    }
}
impl<'a, F: Monad<'a>, T: Mirror<'a, Family = F>> MonadSyntax<'a, F> for T {}

pub trait MonadCloneSyntax<'a, Mon: MonadClone<'a>>:
    Mirror<'a, Family = Mon>
{
    fn flat_map_<B: 'a, F: FnMut(&Self::T) -> Mon::Member<B> + 'a>(
        &self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map_(self.as_member_(), f)
    }
}
impl<'a, F: MonadClone<'a>, T: Mirror<'a, Family = F>> MonadCloneSyntax<'a, F>
    for T
{
}
