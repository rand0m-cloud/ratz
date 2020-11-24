use crate::{associative_flatten::*, covariant::*, hkt::*};

pub trait Monad: AssociativeFlatten + Covariant {
    fn flat_map<'a, 'b, A: 'a, B: 'b, F: FnMut(A) -> Self::Member<B> + 'b>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map(fa, f))
    }
}
pub trait MonadClone: Monad + CovariantClone {
    fn flat_map_<'a, 'b, A: 'a, B: 'b, F: FnMut(&A) -> Self::Member<B> + 'b>(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map_(fa, f))
    }
}
impl<'a, T: AssociativeFlatten + Covariant> Monad for T {}

pub trait MonadSyntax<Mon: Monad>: Mirror<Family = Mon> {
    fn flat_map<'b, B: 'b, F: FnMut(Self::T) -> Mon::Member<B> + 'b>(
        self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map::<Self::T, B, F>(self.as_member(), f)
    }
}
impl<F: Monad, T: Mirror<Family = F>> MonadSyntax<F> for T {}

pub trait MonadCloneSyntax<Mon: MonadClone>: Mirror<Family = Mon> {
    fn flat_map_<'b, B: 'b, F: FnMut(&Self::T) -> Mon::Member<B> + 'b>(
        &self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::flat_map_(self.as_member_(), f)
    }
}
impl<F: MonadClone, T: Mirror<Family = F>> MonadCloneSyntax<F> for T {}
