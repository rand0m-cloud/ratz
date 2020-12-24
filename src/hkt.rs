pub trait Hkt {
    type Member<T>: Mirror<T = T, Family = Self>;
}

pub trait Mirror {
    type T;
    type Family: HktAccepting<Self::T, GetMember = Self>;
}

/// Credit to staffehn for coming up with the approach
/// https://www.reddit.com/r/rust/comments/k88ia8/monads_and_gats_in_nightly_rust/gfe86dk?utm_source=share&utm_medium=web2x&context=3
pub trait HktAccepting<T>: Hkt {
    type GetMember;
}

impl<F: Hkt + ?Sized, T> HktAccepting<T> for F {
    type GetMember = Self::Member<T>;
}

pub trait MirrorExt: Mirror {
    fn as_member(self) -> <Self::Family as Hkt>::Member<Self::T>
    where
        Self: Sized,
    {
        fn identity<F: Hkt, T>(
            x: <F as HktAccepting<T>>::GetMember,
        ) -> F::Member<T> {
            // at least with the explicit type signature above,
            // rustc understands that both types are equal
            x
        }
        identity::<Self::Family, _>(self)
    }
}

impl<T: Mirror + ?Sized> MirrorExt for T {}

pub trait Hkt2 {
    type Member<A, B>: Mirror2<A = A, B = B, Family = Self>;
}

pub trait Mirror2: Sized {
    type A;
    type B;
    type Family: Hkt2;
    fn as_member(self) -> <Self::Family as Hkt2>::Member<Self::A, Self::B>;
}
