use super::hkt::*;
pub trait Contravariant<'a>: Hkt<'a> {
    fn map<A: 'a, B: 'a, F: FnMut(B) -> A + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
