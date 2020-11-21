use super::{covariant::Covariant, identity_both::IdentityBoth};

pub trait Traversable<'a>: Covariant<'a> {
    fn foreach<
        G: IdentityBoth<'a> + Covariant<'a>,
        A: 'a,
        B: 'a,
        F: FnMut(A) -> G::Member<B>,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> G::Member<Self::Member<B>>;
}
