use super::right_covariant::*;

pub trait Profunctor<'family>: RightCovariant<'family> {
    fn dimap<
        'input,
        'output,
        A: 'input,
        B: 'input,
        C: 'output,
        D: 'output,
        F: FnMut(C) -> A + 'output,
        G: FnMut(B) -> D + 'output,
    >(
        ab: Self::Member<'input, A, B>,
        f: F,
        g: G,
    ) -> Self::Member<'output, C, D>;
}
