use super::hkt::*;

pub trait AssociativeFlatten: Hkt {
    fn flatten<A>(ffa: Self::Member<Self::Member<A>>) -> Self::Member<A>;
}
