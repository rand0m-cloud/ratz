pub trait Associative {
    fn combine(self, other: Self) -> Self;
}
