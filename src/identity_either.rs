use super::associative_either::AssociativeEither;

pub trait IdentityEither: AssociativeEither {
    fn none() -> Self::Member<!>;
}
