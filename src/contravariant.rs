use super::hkt::*;
pub trait Contravariant<'a>: Hkt<'a> {
    fn map<A, B, F: Fn(B) -> A>(fa: Self::Member<A>, f: F) -> Self::Member<B>;
}
