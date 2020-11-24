use super::hkt::Hkt;
pub trait AssociativeBoth: Hkt {
    fn both<A, B>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<(A, B)>;
}
