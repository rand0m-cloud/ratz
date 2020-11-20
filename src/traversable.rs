use super::{covariant::Covariant, family::*, identity_both::IdentityBoth};

trait Traversable<'a>: Covariant<'a> {
    fn foreach<
        G: IdentityBoth<'a> + Covariant<'a>,
        A: 'a,
        B: 'a,
        F: FnMut(A) -> G::Member<B>,
    >(
        fa: Self::Member<A>,
        f: F,
    ) -> G::Member<Self::Member<B>>;
}

impl<'a> Traversable<'a> for OptionFamily {
    fn foreach<
        G: IdentityBoth<'a> + Covariant<'a>,
        A: 'a,
        B: 'a,
        F: FnMut(A) -> G::Member<B>,
    >(
        fa: Option<A>,
        mut f: F,
    ) -> G::Member<Option<B>> {
        match fa {
            None => G::map(G::any(), move |_| None),
            Some(x) => G::map(f(x), move |x| Some(x)),
        }
    }
}
