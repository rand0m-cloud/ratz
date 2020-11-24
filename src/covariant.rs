use super::hkt::*;
pub trait Covariant: Hkt {
    fn map<A: 'static, B, F: FnMut(A) -> B + 'static>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
pub trait CovariantClone: Covariant {
    fn map_<A: 'static, B, F: FnMut(&A) -> B + 'static>(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}

pub trait CovariantSyntax<Cov: Covariant, A: 'static>:
    Mirror<T = A, Family = Cov>
{
    fn map<B, F: FnMut(Self::T) -> B + 'static>(self, f: F) -> Cov::Member<B> {
        Cov::map(self.as_member(), f)
    }
}

impl<F: Covariant, A: 'static, T: Mirror<T = A, Family = F>>
    CovariantSyntax<F, A> for T
{
}

pub trait CovariantCloneSyntax<Cov: CovariantClone, A: 'static>:
    Mirror<T = A, Family = Cov>
{
    fn map_<B, F: FnMut(&Self::T) -> B + 'static>(
        &self,
        f: F,
    ) -> Cov::Member<B> {
        Cov::map_(self.as_member_(), f)
    }
}

impl<F: CovariantClone, A: 'static, T: Mirror<T = A, Family = F>>
    CovariantCloneSyntax<F, A> for T
{
}
