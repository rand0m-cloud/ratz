use super::{core::*, hkt::*};

trait AssociativeEither<'a>: Hkt<'a> {
    fn either<A: 'a, B: 'a>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<Either<A, B>>;
}
