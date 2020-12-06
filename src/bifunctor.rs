use super::right_covariant::*;

pub trait BiFunctor: RightCovariant {
    fn bimap<A, B, C, D, F: Fn(A) -> C, G: Fn(B) -> D>(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
