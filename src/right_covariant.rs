use super::hkt::*;

pub trait RightCovariant<'a>: Hkt2<'a> {
    fn right_map<A: 'a, B: 'a, C: 'a, F: FnMut(B) -> C + 'a>(
        ab: Self::Member<A, B>,
        f: F,
    ) -> Self::Member<A, C>;
}
