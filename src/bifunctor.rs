use super::right_covariant::*;

pub trait BiFunctor: RightCovariant {
    fn bimap<A, B, C, D, F: FnMut(A) -> C, G: FnMut(B) -> D>(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
