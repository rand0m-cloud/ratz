use crate::{associative_flatten::*, covariant::*, hkt::*};

pub trait Monad: AssociativeFlatten + Covariant {
    fn flat_map<A, B, F: Fn(A) -> Self::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B> {
        Self::flatten(Self::map(fa, f))
    }
}
impl<T: AssociativeFlatten + Covariant> Monad for T {}

pub trait MonadExt: Mirror + Sized
where
    Self::Family: Monad,
{
    fn flat_map<B, F: Fn(Self::T) -> <Self::Family as Hkt>::Member<B>>(
        self,
        f: F,
    ) -> <Self::Family as Hkt>::Member<B> {
        <Self::Family as Monad>::flat_map(self.as_member(), f)
    }
}

impl<F: Monad, A, T: Mirror<Family = F, T = A>> MonadExt for T {}
