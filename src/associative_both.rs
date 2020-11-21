use super::hkt::Hkt;
pub trait AssociativeBoth<'a>: Hkt<'a> {
    fn both<A: 'a, B: 'a>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<(A, B)>;
}
