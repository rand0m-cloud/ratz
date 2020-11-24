use super::right_covariant::*;

pub trait BiFunctor<'family>: RightCovariant<'family> {
    fn bimap<
        'i,
        'o,
        A: 'i,
        B: 'i,
        C: 'o,
        D: 'o,
        F: FnMut(A) -> C + 'o,
        G: FnMut(B) -> D + 'o,
    >(
        ab: Self::Member<'i, A, B>,
        f: F,
        g: G,
    ) -> Self::Member<'o, C, D>;
}
