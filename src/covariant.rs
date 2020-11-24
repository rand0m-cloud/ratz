use super::hkt::*;
pub trait Covariant<'a>: Hkt<'a> {
    fn map<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
pub trait CovariantClone<'a>: Covariant<'a> {
    fn map_<A: 'a, B: 'a, F: FnMut(&A) -> B + 'a>(
        fa: &Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}

pub trait CovariantSyntax<'a, Cov: Covariant<'a>>:
    Mirror<'a, Family = Cov>
{
    fn map<B: 'a, F: FnMut(Self::T) -> B + 'a>(self, f: F) -> Cov::Member<B> {
        Cov::map(self.as_member(), f)
    }
}

impl<'a, F: Covariant<'a>, T: Mirror<'a, Family = F>> CovariantSyntax<'a, F>
    for T
{
}

pub trait CovariantCloneSyntax<'a, Cov: CovariantClone<'a>>:
    Mirror<'a, Family = Cov>
{
    fn map_<B: 'a, F: FnMut(&Self::T) -> B + 'a>(
        &self,
        f: F,
    ) -> Cov::Member<B> {
        Cov::map_(self.as_member_(), f)
    }
}

impl<'a, F: CovariantClone<'a>, T: Mirror<'a, Family = F>>
    CovariantCloneSyntax<'a, F> for T
{
}
