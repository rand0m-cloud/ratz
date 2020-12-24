use crate::dev::*;

impl<A> Mirror for Vec<A> {
    type Family = VecFamily;
    type T = A;
}

pub struct VecFamily;

impl Hkt for VecFamily {
    type Member<T> = Vec<T>;
}

impl Covariant for VecFamily {
    fn map<A, B, F: Fn(A) -> B>(fa: Vec<A>, f: F) -> Vec<B> {
        fa.map(f)
    }
}

impl AssociativeFlatten for VecFamily {
    fn flatten<A>(ffa: Vec<Vec<A>>) -> Vec<A> {
        ffa.into_iter().flatten().collect()
    }
}
