use super::{core::*, family::*};

trait AssociativeEither<'a>: Family<'a> {
    fn either<A: 'a, B: 'a>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<Either<A, B>>;
}
