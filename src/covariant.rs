use super::hkt::*;
pub trait Covariant: Hkt {
    fn map<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
pub trait CovariantClone: Covariant {
    fn map_<'a, A: 'a, B: 'a, F: FnMut(&A) -> B + 'a>(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}

pub trait CovariantSyntax<Cov: Covariant>: Mirror<Family = Cov> {
    fn map<'a, B: 'a, F: FnMut(Self::T) -> B + 'a>(
        self,
        f: F,
    ) -> Cov::Member<B> {
        Cov::map(self.as_member(), f)
    }
}

impl<F: Covariant, T: Mirror<Family = F>> CovariantSyntax<F> for T {}

pub trait CovariantCloneSyntax<Cov: CovariantClone>:
    Mirror<Family = Cov>
{
    fn map_<'a, B: 'a, F: FnMut(&Self::T) -> B + 'a>(
        &self,
        f: F,
    ) -> Cov::Member<B> {
        Cov::map_(self.as_member_(), f)
    }
}

impl<'t, F: CovariantClone, T: Mirror<Family = F>> CovariantCloneSyntax<F>
    for T
{
}
