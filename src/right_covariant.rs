use super::hkt::*;

pub trait RightCovariant: Hkt2 {
    fn right_map<
        'c,
        A: 'static,
        B: 'static,
        C: 'c,
        F: FnMut(B) -> C + 'static,
    >(
        ab: Self::Member<A, B>,
        f: F,
    ) -> Self::Member<A, C>;
}
