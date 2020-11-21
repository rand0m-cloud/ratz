use super::right_covariant::*;

pub trait Profunctor<'a>: RightCovariant<'a> {
    fn dimap<
        A: 'a,
        B: 'a,
        C: 'a,
        D: 'a,
        F: FnMut(C) -> A + 'a,
        G: FnMut(B) -> D + 'a,
    >(
        ab: Self::Member<A, B>,
        f: F,
        g: G,
    ) -> Self::Member<C, D>;
}
