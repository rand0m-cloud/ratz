trait Semigroup {
    fn compose(self, other: Self) -> Self;
}
trait Monoid {
    fn empty() -> Self;
}
trait Hkt {
    type Unplug;
    type Plug<B>: Hkt;
}
trait Functor: Hkt {
    fn fmap<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> B;
}
trait Applicative: Functor {
    fn pure<A>(a: A) -> Self::Plug<A>;
    fn azip<B>(self, fb: Self::Plug<B>) -> Self::Plug<(Self::Unplug, B)>;
}
trait Monad: Applicative {
    fn bind<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> Self::Plug<B>;
}
trait Foldable: Hkt {
    fn fold_map<M, F>(self, f: F) -> M
    where
        M: Monoid,
        F: Fn(Self::Unplug) -> M;
}
trait Traversable: Foldable {
    fn traverse<B, AppB, F>(self, f: F) -> AppB::Plug<Self::Plug<B>>
    where
        AppB: Applicative<Unplug = B>,
        F: Fn(Self::Unplug) -> AppB;
}

impl<A> Hkt for Option<A> {
    type Plug<B> = Option<B>;
    type Unplug = A;
}

impl<A> Functor for Option<A> {
    fn fmap<B, F>(self, f: F) -> Self::Plug<B>
    where
        F: Fn(Self::Unplug) -> B,
    {
        self.map(f)
    }
}
impl<A> Applicative for Option<A> {
    fn pure<T>(a: T) -> Self::Plug<T> {
        Some(a)
    }

    fn azip<B>(self, fb: Option<B>) -> Option<(A, B)> {
        match (self, fb) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}
impl<A> Monad for Option<A> {
    fn bind<B, F>(self, f: F) -> Option<B>
    where
        F: Fn(A) -> Option<B>,
    {
        self.and_then(f)
    }
}
impl<A> Foldable for Option<A> {
    fn fold_map<M, F>(self, f: F) -> M
    where
        M: Monoid,
        F: Fn(A) -> M,
    {
        match self {
            Some(a) => f(a),
            None => M::empty(),
        }
    }
}
impl<A> Traversable for Option<A> {
    fn traverse<B, AppB, F>(self, f: F) -> AppB::Plug<Option<B>>
    where
        AppB: Applicative<Unplug = B>,
        F: Fn(A) -> AppB,
    {
        match self {
            None => AppB::pure(None),
            Some(a) => f(a).fmap(|x| Some(x)),
        }
    }
}

fn main() {
    let x = Some(1).bind(|x| Some(x * 2));
    println!("{:?}", x);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zip() {
        let zipped = Some(1).azip(None as Option<bool>);
        assert_eq!(zipped, None);
    }
}
