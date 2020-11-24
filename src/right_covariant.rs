use super::hkt::*;

pub trait RightCovariant: Hkt2 {
    fn right_map<A, B, C, F: FnMut(B) -> C>(
        ab: Self::Member<A, B>,
        f: F,
    ) -> Self::Member<A, C>;
}
