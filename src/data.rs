use super::{
    associative_both::*,
    associative_flatten::*,
    covariant::*,
    derived::monad::*,
    hkt::*,
};
use std::marker::PhantomData;
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct EitherFamily<'a, A: 'a>(PhantomData<&'a A>);
impl<'a, A: 'a> Hkt<'a> for EitherFamily<'a, A> {
    type Member<B: 'a> = Either<A, B>;
}
impl<'a, A: 'a, B: 'a> Mirror<'a> for Either<A, B> {
    type Family = EitherFamily<'a, A>;
    type T = B;

    fn as_member(self) -> <Self::Family as Hkt<'a>>::Member<Self::T> {
        self
    }
}
impl<'a, L: 'a> Covariant<'a> for EitherFamily<'a, L> {
    fn map<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        mut f: F,
    ) -> Self::Member<B> {
        match fa {
            Either::Left(a) => Either::Left(a),
            Either::Right(b) => Either::Right(f(b)),
        }
    }
}
impl<'a, L: 'a> AssociativeFlatten<'a> for EitherFamily<'a, L> {
    fn flatten<A: 'a>(ffa: Self::Member<Self::Member<A>>) -> Self::Member<A> {
        match ffa {
            Either::Left(l) => Either::Left(l),
            Either::Right(fa) => fa,
        }
    }
}
// impl<'a, L: Clone + 'a> AssociativeBoth<'a> for EitherFamily<'a, L> {
//     fn both<A: 'a, B: 'a>(
//         mut fa: Either<L, A>,
//         mut fb: Either<L, B>,
//     ) -> Either<L, (A, B)> {
//         fa.flat_map(move |a| fb.map(move |b| (a, b)))
//     }
// }
