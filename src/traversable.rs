use super::{covariant::Covariant, hkt::*, identity_both::IdentityBoth};

pub trait Traversable: Covariant {
    fn foreach<
        'b,
        App: IdentityBoth + Covariant,
        A,
        B: 'b,
        F: FnMut(A) -> App::Member<B> + 'b,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}

pub trait TraverseSyntax<Tr: Traversable>: Mirror<Family = Tr> {
    fn traverse<
        'b,
        App: IdentityBoth + Covariant,
        AppB: Mirror<Family = App>,
        F: FnMut(Self::T) -> AppB + 'b,
    >(
        self,
        mut f: F,
    ) -> App::Member<Tr::Member<AppB::T>> {
        Tr::foreach::<App, _, _, _>(self.as_member(), move |t| f(t).as_member())
    }
}
impl<F: Traversable, T: Mirror<Family = F>> TraverseSyntax<F> for T {}

pub trait SequenceSyntax<
    App: IdentityBoth + Covariant,
    Tr: Traversable,
    AppA: Mirror<Family = App>,
>: Mirror<T = AppA, Family = Tr>
{
    fn sequence(self) -> App::Member<Tr::Member<AppA::T>> {
        self.traverse(move |x| x)
    }
}
impl<
        App: IdentityBoth + Covariant,
        Tr: Traversable,
        AppA: Mirror<Family = App>,
        TrAppA: Mirror<T = AppA, Family = Tr>,
    > SequenceSyntax<App, Tr, AppA> for TrAppA
{
}
