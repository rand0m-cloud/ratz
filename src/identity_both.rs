use super::associative_both::AssociativeBoth;

pub trait IdentityBoth: AssociativeBoth {
    fn unit() -> Self::Member<()>;
}
