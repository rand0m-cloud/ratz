use super::hkt::Hkt;
pub trait AssociativeBoth: Hkt {
    fn both<A: Clone, B: Clone>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<(A, B)>;
}
