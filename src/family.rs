use std::marker::PhantomData;
pub trait Family<'a> {
    type Member<T: 'a>: Mirror<'a, T = T, Family = Self>;
}
pub trait Mirror<'a>: Sized + 'a {
    type T: 'a;
    type Family: Family<'a>;
    fn as_member(self) -> <Self::Family as Family<'a>>::Member<Self::T>;
}

pub struct OptionFamily;
impl<'a> Family<'a> for OptionFamily {
    type Member<T: 'a> = Option<T>;
}
impl<'a, A: 'a> Mirror<'a> for Option<A> {
    type Family = OptionFamily;
    type T = A;

    fn as_member(self) -> Option<A> {
        self
    }
}

pub struct VectorFamily;
impl<'a> Family<'a> for VectorFamily {
    type Member<T: 'a> = Vec<T>;
}
impl<'a, A: 'a> Mirror<'a> for Vec<A> {
    type Family = VectorFamily;
    type T = A;

    fn as_member(self) -> Vec<A> {
        self
    }
}

pub struct ResultFamily<'a, E: 'a> {
    phantom: PhantomData<&'a E>,
}
impl<'a, E: 'a> Family<'a> for ResultFamily<'a, E> {
    type Member<T: 'a> = Result<T, E>;
}
impl<'a, A: 'a, E: 'a> Mirror<'a> for Result<A, E> {
    type Family = ResultFamily<'a, E>;
    type T = A;

    fn as_member(self) -> Result<A, E> {
        self
    }
}
