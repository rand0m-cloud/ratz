use super::associative_both::AssociativeBoth;

pub trait IdentityBoth<'a>: AssociativeBoth<'a> {
    fn any() -> Self::Member<()>;
}
