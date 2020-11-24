use super::hkt::Hkt;
pub trait AssociativeBoth: Hkt {
    fn both<'a, A: 'a, B: 'a>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<(A, B)>;
}
