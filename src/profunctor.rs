use super::right_covariant::*;

pub trait Profunctor: RightCovariant {
    fn dimap<A, B, C, D, F: Fn(C) -> A, G: Fn(B) -> D>(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
