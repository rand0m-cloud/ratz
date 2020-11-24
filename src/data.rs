use super::{associative_flatten::*, covariant::*, derived::monad::*, hkt::*};
use std::marker::PhantomData;

// iterator
impl<T> Mirror for Box<dyn Iterator<Item = T>> {
    type Family = IteratorBoxFamily;
    type T = T;

    fn as_member(self) -> <Self::Family as Hkt>::Member<Self::T> {
        self
    }

    fn as_member_(&self) -> &<Self::Family as Hkt>::Member<Self::T> {
        self
    }
}
struct IteratorBoxFamily;
impl Hkt for IteratorBoxFamily {
    type Member<T> = Box<dyn Iterator<Item = T>>;
}
impl Covariant for IteratorBoxFamily {
    fn map<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Box<dyn Iterator<Item = A>>,
        f: F,
    ) -> Box<dyn Iterator<Item = B> + 'a> {
        box Iterator::map(fa, f)
    }
}

// either
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct EitherFamily<L>(PhantomData<L>);
impl<L> Hkt for EitherFamily<L> {
    type Member<R> = Either<L, R>;
}
impl<L, R> Mirror for Either<L, R> {
    type Family = EitherFamily<L>;
    type T = R;

    fn as_member(self) -> Either<L, R> {
        self
    }

    fn as_member_(&self) -> &Either<L, R> {
        self
    }
}
impl<L> Covariant for EitherFamily<L> {
    fn map<'a, A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Either<L, A>,
        mut f: F,
    ) -> Either<L, B> {
        match fa {
            Either::Left(a) => Either::Left(a),
            Either::Right(b) => Either::Right(f(b)),
        }
    }
}
impl<L: Clone> CovariantClone for EitherFamily<L> {
    fn map_<'a, A: 'a, B: 'a, F: FnMut(&A) -> B + 'a>(
        fa: &Either<L, A>,
        mut f: F,
    ) -> Either<L, B> {
        match fa {
            Either::Left(a) => Either::Left(a.clone()),
            Either::Right(b) => Either::Right(f(b)),
        }
    }
}
impl<L> AssociativeFlatten for EitherFamily<L> {
    fn flatten<'a, A: 'a>(ffa: Either<L, Either<L, A>>) -> Either<L, A> {
        match ffa {
            Either::Left(l) => Either::Left(l),
            Either::Right(fa) => fa,
        }
    }
}
// impl<'a, L: Clone + 'a> AssociativeBoth<'a> for EitherFamily<'a, L> {
//     fn both<A: 'a, B: 'a>(
//         fa: Either<L, A>,
//         fb: Either<L, B>,
//     ) -> Either<L, (A, B)> {
//         fa.flat_map(move |a| fb.map(move |b| (a, b)))
//     }
// }
