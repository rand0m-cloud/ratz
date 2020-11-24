use super::{data::*, hkt::*};

pub trait AssociativeEither: Hkt {
    fn either<'a, A: 'a, B: 'a>(
        fa: Self::Member<'a, A>,
        fb: Self::Member<'a, B>,
    ) -> Self::Member<'a, Either<A, B>>;
}
