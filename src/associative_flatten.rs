use super::hkt::*;

pub trait AssociativeFlatten: Hkt {
    fn flatten<'a, A: 'a>(
        ffa: Self::Member<Self::Member<A>>,
    ) -> Self::Member<A>;
}
