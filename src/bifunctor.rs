use super::right_covariant::*;

pub trait BiFunctor<'a>: RightCovariant<'a> {
    fn bimap<
        A: 'a,
        B: 'a,
        C: 'a,
        D: 'a,
        F: FnMut(A) -> C + 'a,
        G: FnMut(B) -> D + 'a,
    >(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
