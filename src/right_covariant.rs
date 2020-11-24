use super::hkt::*;

pub trait RightCovariant<'family>: Hkt2<'family> {
    fn right_map<
        'input,
        'output,
        A: 'input,
        B: 'output,
        C: 'output,
        F: FnMut(B) -> C + 'output,
    >(
        ab: Self::Member<'input, A, B>,
        f: F,
    ) -> Self::Member<'output, A, C>;
}
