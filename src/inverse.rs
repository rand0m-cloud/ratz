use super::identity::Identity;
pub trait Inverse: Identity {
    fn inverse(self, other: Self) -> Self;
}
