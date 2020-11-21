use super::core::*;

pub trait Hkt<'a> {
    type Member<T: 'a>: Mirror<'a, T = T, Family = Self>;
}
pub trait Mirror<'a>: Sized + 'a {
    type T: 'a;
    type Family: Hkt<'a>;
    fn as_member(self) -> <Self::Family as Hkt<'a>>::Member<Self::T>;
}

impl<'a> Hkt<'a> for OptionFamily {
    type Member<T: 'a> = Option<T>;
}
impl<'a, A: 'a> Mirror<'a> for Option<A> {
    type Family = OptionFamily;
    type T = A;

    fn as_member(self) -> Option<A> {
        self
    }
}

impl<'a> Hkt<'a> for VectorFamily {
    type Member<T: 'a> = Vec<T>;
}
impl<'a, A: 'a> Mirror<'a> for Vec<A> {
    type Family = VectorFamily;
    type T = A;

    fn as_member(self) -> Vec<A> {
        self
    }
}

impl<'a, E: 'a> Hkt<'a> for ResultFamily<'a, E> {
    type Member<T: 'a> = Result<T, E>;
}
impl<'a, A: 'a, E: 'a> Mirror<'a> for Result<A, E> {
    type Family = ResultFamily<'a, E>;
    type T = A;

    fn as_member(self) -> Result<A, E> {
        self
    }
}
