pub trait Hkt {
    type Member<T>: Mirror<T = T, Family = Self>;
}

pub trait Mirror: Sized {
    type T;
    type Family: Hkt;
    fn as_member(self) -> <Self::Family as Hkt>::Member<Self::T>;
    fn as_member_(&self) -> &<Self::Family as Hkt>::Member<Self::T>;
}

pub trait Hkt2<'family> {
    type Member<'member, A: 'member, B: 'member>: Mirror2<
        'family,
        'member,
        A = A,
        B = B,
        Family = Self,
    >;
}

pub trait Mirror2<'member, 'family>: Sized + 'member + 'family {
    type A: 'member;
    type B: 'member;
    type Family: Hkt2<'family>;
    fn as_member(
        self,
    ) -> <Self::Family as Hkt2<'family>>::Member<'member, Self::A, Self::B>;
    fn as_member_(
        &self,
    ) -> &<Self::Family as Hkt2<'family>>::Member<'member, Self::A, Self::B>;
}
