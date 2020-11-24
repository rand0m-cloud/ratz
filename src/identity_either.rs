use super::associative_either::AssociativeEither;

pub trait IdentityBoth: AssociativeBoth<'a> {
    fn none() -> Self::Member<!>;
}
