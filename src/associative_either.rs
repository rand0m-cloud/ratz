use crate::dev::*;

pub trait AssociativeEither: Hkt {
    fn either<A, B>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<Either<A, B>>;
}
