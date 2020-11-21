use super::hkt::*;

pub trait AssociativeFlatten<'a>: Hkt<'a> {
    fn flatten<A: 'a>(ffa: Self::Member<Self::Member<A>>) -> Self::Member<A>;
}
