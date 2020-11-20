use super::family::Family;
pub trait AssociativeBoth<'a>: Family<'a> {
    fn both<A: 'a, B: 'a>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<(A, B)>;
}
