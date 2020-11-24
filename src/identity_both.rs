use super::associative_both::AssociativeBoth;

pub trait IdentityBoth: AssociativeBoth {
    fn any() -> Self::Member<()>;
}
