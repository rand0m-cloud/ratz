use super::hkt::*;
pub trait Contravariant<'a>: Hkt<'a> {
    fn map<A, B, F: FnMut(B) -> A>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
