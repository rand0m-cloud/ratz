use super::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}
