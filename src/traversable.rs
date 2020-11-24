use super::{covariant::*, derived::applicative::*, hkt::*};

pub trait Traversable: Covariant {
    fn foreach<
        App: Applicative,
        A: 'static,
        B: Clone + 'static,
        F: FnMut(A) -> App::Member<B> + 'static,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}

pub trait TraverseSyntax<Tr: Traversable, A: 'static>:
    Mirror<Family = Tr, T = A>
{
    fn traverse<
        App: Applicative,
        B: Clone + 'static,
        AppB: Mirror<Family = App, T = B>,
        F: FnMut(Self::T) -> AppB + 'static,
    >(
        self,
        mut f: F,
    ) -> App::Member<Tr::Member<B>> {
        Tr::foreach::<App, _, _, _>(self.as_member(), move |t| f(t).as_member())
    }
}
impl<F: Traversable, A: 'static, T: Mirror<Family = F, T = A>>
    TraverseSyntax<F, A> for T
{
}

pub trait SequenceSyntax<
    App: Applicative,
    Tr: Traversable,
    A: Clone + 'static,
    AppA: Mirror<Family = App, T = A> + 'static,
>: Mirror<Family = Tr, T = AppA>
{
    fn sequence(self) -> App::Member<Tr::Member<A>> {
        self.traverse(move |x| x)
    }
}
impl<
        App: Applicative,
        Tr: Traversable,
        A: Clone + 'static,
        AppA: Mirror<Family = App, T = A> + 'static,
        TrAppA: Mirror<Family = Tr, T = AppA>,
    > SequenceSyntax<App, Tr, A, AppA> for TrAppA
{
}
