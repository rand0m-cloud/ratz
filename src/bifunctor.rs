use super::right_covariant::*;

pub trait BiFunctor: RightCovariant {
    fn bimap<
        'output,
        A: 'static,
        B: 'static,
        C: 'output,
        D: 'output,
        F: FnMut(A) -> C + 'static,
        G: FnMut(B) -> D + 'static,
    >(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
