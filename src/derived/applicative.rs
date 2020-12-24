use crate::{covariant::*, identity_both::*};

pub trait Applicative: Covariant + IdentityBoth {
    fn pure<A: Clone>(a: A) -> Self::Member<A> {
        Self::map(Self::unit(), move |_| a.clone())
    }
}
impl<T: Covariant + IdentityBoth> Applicative for T {}
