use super::associative_either::AssociativeEither;

pub trait IdentityBoth<'a>: AssociativeBoth<'a> {
    fn none() -> Self::Member<!>;
}
