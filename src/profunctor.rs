use super::right_covariant::*;

pub trait Profunctor: RightCovariant {
    fn dimap<A, B, C, D, F: FnMut(C) -> A, G: FnMut(B) -> D>(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
