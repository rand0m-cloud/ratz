use super::{associative_flatten::*, covariant::*, family::*};

pub trait Monad<'a>: AssociativeFlatten<'a> + Covariant<'a> {
    fn bind<A: 'a, B: 'a, F: FnMut(A) -> Self::Member<B> + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map(fa, f))
    }
}
impl<'a, T: AssociativeFlatten<'a> + Covariant<'a>> Monad<'a> for T {}

pub trait MonadSyntax<'a, Mon: Monad<'a>>: Mirror<'a, Family = Mon> {
    fn bind<B: 'a, F: FnMut(Self::T) -> Mon::Member<B> + 'a>(
        self,
        f: F,
    ) -> Mon::Member<B> {
        Mon::bind(self.as_member(), f)
    }
}
impl<'a, F: Monad<'a>, T: Mirror<'a, Family = F>> MonadSyntax<'a, F> for T {}
