pub trait Hkt {
    type Member<T>: Mirror<T = T, Family = Self>;
}

pub trait Mirror: Sized {
    type T;
    type Family: Hkt;
    fn as_member(self) -> <Self::Family as Hkt>::Member<Self::T>;
    fn as_member_(&self) -> &<Self::Family as Hkt>::Member<Self::T>;
}

pub trait Hkt2 {
    type Member<A, B>: Mirror2<A = A, B = B, Family = Self>;
}

pub trait Mirror2: Sized {
    type A;
    type B;
    type Family: Hkt2;
    fn as_member(self) -> <Self::Family as Hkt2>::Member<Self::A, Self::B>;
    fn as_member_(&self) -> &<Self::Family as Hkt2>::Member<Self::A, Self::B>;
}
