use super::hkt::*;

pub trait AssociativeFlatten: Hkt {
    fn flatten<A: 'static>(
        ffa: Self::Member<Self::Member<A>>,
    ) -> Self::Member<A>;
}
