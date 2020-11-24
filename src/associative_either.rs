use super::{data::*, hkt::*};

pub trait AssociativeEither: Hkt {
    fn either<A: 'static, B: 'static>(
        fa: Self::Member<A>,
        fb: Self::Member<B>,
    ) -> Self::Member<Either<A, B>>;
}
