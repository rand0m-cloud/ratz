use super::right_covariant::*;

pub trait Profunctor: RightCovariant {
    fn dimap<
        'output,
        A: 'static,
        B: 'static,
        C: 'output,
        D: 'output,
        F: FnMut(C) -> A + 'static,
        G: FnMut(B) -> D + 'static,
    >(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
