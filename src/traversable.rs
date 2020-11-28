use super::{covariant::*, derived::applicative::*, hkt::*};

pub trait Traversable: Covariant {
    fn foreach<App: Applicative, A, B: Clone, F: FnMut(A) -> App::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}
pub trait TraversableClone: Traversable + CovariantClone {
    fn foreach_<
        'a,
        'b: 'a,
        App: Applicative + 'b,
        A: 'a,
        B: Clone + 'b,
        F: FnMut(&'a A) -> &'b App::Member<B>,
    >(
        fa: &'a Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}

pub trait TraverseSyntax<Tr: Traversable, A>:
    Mirror<Family = Tr, T = A>
{
    fn traverse<
        App: Applicative,
        B: Clone,
        AppB: Mirror<Family = App, T = B>,
        F: FnMut(Self::T) -> AppB,
    >(
        self,
        mut f: F,
    ) -> App::Member<Tr::Member<B>> {
        Tr::foreach::<App, _, _, _>(self.as_member(), move |t| f(t).as_member())
    }
}
impl<F: Traversable, A, T: Mirror<Family = F, T = A>> TraverseSyntax<F, A>
    for T
{
}
pub trait TraverseCloneSyntax<'a, 'b: 'a, Tr: TraversableClone + 'b, A: 'a>:
    Mirror<Family = Tr, T = A>
{
    fn traverse_<
        App: Applicative + 'b,
        B: Clone + 'b,
        AppB: Mirror<Family = App, T = B> + 'b,
        F: FnMut(&'a Self::T) -> &'b AppB,
    >(
        &'a self,
        mut f: F,
    ) -> App::Member<Tr::Member<B>> {
        Tr::foreach_::<App, _, _, _>(self.as_member_(), move |t| {
            f(t).as_member_()
        })
    }
}
impl<
        'a,
        'b: 'a,
        F: TraversableClone + 'b,
        A: 'a,
        T: Mirror<Family = F, T = A>,
    > TraverseCloneSyntax<'a, 'b, F, A> for T
{
}

pub trait SequenceSyntax<
    App: Applicative,
    Tr: Traversable,
    A: Clone,
    AppA: Mirror<Family = App, T = A>,
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
        AppA: Mirror<Family = App, T = A>,
        TrAppA: Mirror<Family = Tr, T = AppA>,
    > SequenceSyntax<App, Tr, A, AppA> for TrAppA
{
}
pub trait SequenceCloneSyntax<
    'a,
    App: Applicative + 'a,
    Tr: TraversableClone + 'a,
    A: Clone + 'a,
    AppA: Mirror<Family = App, T = A> + 'a,
>: Mirror<Family = Tr, T = AppA>
{
    fn sequence_(&'a self) -> App::Member<Tr::Member<A>> {
        self.traverse_(move |x| x)
    }
}
impl<
        'a,
        App: Applicative + 'a,
        Tr: TraversableClone + 'a,
        A: Clone + 'a,
        AppA: Mirror<Family = App, T = A> + 'a,
        TrAppA: Mirror<Family = Tr, T = AppA> + 'a,
    > SequenceCloneSyntax<'a, App, Tr, A, AppA> for TrAppA
{
}
