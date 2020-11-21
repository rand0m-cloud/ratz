use super::core::*;

pub trait Hkt2<'a> {
    type Member<A: 'a, B: 'a>: Mirror2<'a, A = A, B = B, Family = Self>;
}
pub trait Mirror2<'a>: Sized + 'a {
    type A: 'a;
    type B: 'a;
    type Family: Hkt2<'a>;
    fn as_member(self) -> <Self::Family as Hkt2<'a>>::Member<Self::A, Self::B>;
}

impl<'a> Hkt2<'a> for ResultFamily2 {
    type Member<A: 'a, B: 'a> = Result<A, B>;
}
impl<'a, A: 'a, E: 'a> Mirror2<'a> for Result<A, E> {
    type A = A;
    type B = E;
    type Family = ResultFamily2;

    fn as_member(self) -> <Self::Family as Hkt2<'a>>::Member<Self::A, Self::B> {
        self
    }
}
