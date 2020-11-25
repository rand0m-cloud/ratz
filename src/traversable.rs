use super::{covariant::*, derived::applicative::*, hkt::*};

pub trait Traversable: Covariant {
    fn foreach<App: Applicative, A, B: Clone, F: FnMut(A) -> App::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}

pub trait TraverseSyntax<Tr: Traversable, A: Clone>:
    Mirror<Family = Tr, T = A>
{
    fn traverse<
        App: Applicative,
        B: Clone,
        AppB: Mirror<Family = App, T = B> + Clone,
        F: FnMut(Self::T) -> AppB,
    >(
        self,
        mut f: F,
    ) -> App::Member<Tr::Member<B>> {
        Tr::foreach::<App, _, _, _>(self.as_member(), move |t| f(t).as_member())
    }
}
impl<F: Traversable, A: Clone, T: Mirror<Family = F, T = A>>
    TraverseSyntax<F, A> for T
{
}

pub trait SequenceSyntax<
    App: Applicative,
    Tr: Traversable,
    A: Clone,
    AppA: Mirror<Family = App, T = A> + Clone,
>: Mirror<Family = Tr, T = AppA>
{
    fn sequence(self) -> App::Member<Tr::Member<A>> {
        self.traverse(move |x| x)
    }
}
impl<
        App: Applicative,
        Tr: Traversable,
        A: Clone,
        AppA: Mirror<Family = App, T = A> + Clone,
        TrAppA: Mirror<Family = Tr, T = AppA>,
    > SequenceSyntax<App, Tr, A, AppA> for TrAppA
{
}
