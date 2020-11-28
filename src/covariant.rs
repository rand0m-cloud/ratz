use super::hkt::*;
pub trait Covariant: Hkt {
    fn map<A, B, F: FnMut(A) -> B>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
pub trait CovariantSyntax<Cov: Covariant, A>:
    Mirror<T = A, Family = Cov>
{
    fn map<B, F: FnMut(Self::T) -> B>(self, f: F) -> Cov::Member<B> {
        Cov::map(self.as_member(), f)
    }
}

impl<F: Covariant, A, T: Mirror<T = A, Family = F>> CovariantSyntax<F, A>
    for T
{
}
