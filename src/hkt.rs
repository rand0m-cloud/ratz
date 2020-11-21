pub trait Hkt<'a> {
    type Member<T: 'a>: Mirror<'a, T = T, Family = Self>;
}

pub trait Mirror<'a>: Sized + 'a {
    type T: 'a;
    type Family: Hkt<'a>;
    fn as_member(self) -> <Self::Family as Hkt<'a>>::Member<Self::T>;
}

pub trait Hkt2<'a> {
    type Member<A: 'a, B: 'a>: Mirror2<'a, A = A, B = B, Family = Self>;
}

pub trait Mirror2<'a>: Sized + 'a {
    type A: 'a;
    type B: 'a;
    type Family: Hkt2<'a>;
    fn as_member(self) -> <Self::Family as Hkt2<'a>>::Member<Self::A, Self::B>;
}
