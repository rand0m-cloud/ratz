use super::{covariant::Covariant, hkt::*, identity_both::IdentityBoth};

pub trait Traversable<'a>: Covariant<'a> {
    fn foreach<
        G: IdentityBoth<'a> + Covariant<'a>,
        A: 'a,
        B: 'a,
        F: FnMut(A) -> G::Member<B> + 'a,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> G::Member<Self::Member<B>>;
}

pub trait TraverseSyntax<'a, Tr: Traversable<'a>>:
    Mirror<'a, Family = Tr>
{
    fn traverse<
        App: IdentityBoth<'a> + Covariant<'a>,
        AppB: Mirror<'a, Family = App>,
        F: FnMut(Self::T) -> AppB + 'a,
    >(
        self,
        mut f: F,
    ) -> App::Member<Tr::Member<AppB::T>> {
        Tr::foreach::<App, _, _, _>(self.as_member(), move |t| f(t).as_member())
    }
}
impl<'a, F: Traversable<'a>, T: Mirror<'a, Family = F>> TraverseSyntax<'a, F>
    for T
{
}

pub trait SequenceSyntax<
    'a,
    App: IdentityBoth<'a> + Covariant<'a>,
    Tr: Traversable<'a>,
    AppA: Mirror<'a, Family = App>,
>: Mirror<'a, T = AppA, Family = Tr> + Sized
{
    fn sequence(self) -> App::Member<Tr::Member<AppA::T>> {
        self.traverse(|x| x)
    }
}
impl<
        'a,
        App: IdentityBoth<'a> + Covariant<'a>,
        Tr: Traversable<'a>,
        AppA: Mirror<'a, Family = App>,
        TrAppA: Mirror<'a, T = AppA, Family = Tr>,
    > SequenceSyntax<'a, App, Tr, AppA> for TrAppA
{
}
