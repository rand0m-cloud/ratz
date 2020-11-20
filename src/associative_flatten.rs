use super::family::*;

pub trait AssociativeFlatten<'a>: Family<'a> {
    fn flatten<A: 'a>(ffa: Self::Member<Self::Member<A>>) -> Self::Member<A>;
}
