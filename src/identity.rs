use super::associative::Associative;

pub trait Identity: Associative {
    fn identity() -> Self;
}
