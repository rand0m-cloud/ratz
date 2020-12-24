use crate::dev::*;

impl<A> Mirror for Option<A> {
    type Family = OptionFamily;
    type T = A;
}

pub struct OptionFamily;

impl Hkt for OptionFamily {
    type Member<T> = Option<T>;
}

impl Covariant for OptionFamily {
    fn map<A, B, F: Fn(A) -> B>(fa: Option<A>, f: F) -> Option<B> {
        fa.map(f)
    }
}

impl AssociativeFlatten for OptionFamily {
    fn flatten<A>(ffa: Option<Option<A>>) -> Option<A> {
        match ffa {
            Some(fa) => fa,
            None => None,
        }
    }
}

impl AssociativeBoth for OptionFamily {
    fn both<A: Clone, B: Clone>(
        fa: Option<A>,
        fb: Option<B>,
    ) -> Option<(A, B)> {
        match (fa, fb) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

impl AssociativeEither for OptionFamily {
    fn either<A, B>(fa: Option<A>, fb: Option<B>) -> Option<Either<A, B>> {
        fa.map(Either::Left).or_else(|| fb.map(Either::Right))
    }
}

impl IdentityBoth for OptionFamily {
    fn unit() -> Self::Member<()> {
        Some(())
    }
}

impl Traversable for OptionFamily {
    fn traverse<App: Applicative, A, B: Clone, F: Fn(A) -> App::Member<B>>(
        fa: Option<A>,
        f: F,
    ) -> App::Member<Option<B>> {
        match fa {
            Some(a) => f(a).map(|b| Some(b)),
            None => App::pure(None),
        }
    }
}
