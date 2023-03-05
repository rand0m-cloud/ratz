// HKTs with one parameter

pub trait TypeConstructor1 {
  type Of<A>: Mirror1<Constructor = Self, A = A>;
}

pub trait AppliedTypeConstructor1<A>: TypeConstructor1 {
  type Mirror: Mirror1<Constructor = Self, A = A>; // eventually use an associated type default here.
}

impl<F: TypeConstructor1, A> AppliedTypeConstructor1<A> for F {
  type Mirror = Self::Of<A>;
}

pub trait Mirror1: Sized {
  type Constructor: AppliedTypeConstructor1<Self::A, Mirror = Self>;

  type A;

  fn as_member(self) -> <Self::Constructor as TypeConstructor1>::Of<Self::A> {
    fn witness<F: TypeConstructor1, A>(
      fa: <F as AppliedTypeConstructor1<A>>::Mirror,
    ) -> F::Of<A> {
      fa
    }

    witness::<Self::Constructor, Self::A>(self)
  }
}

// HKTs with two parameters

pub trait TypeConstructor2 {
  type Of<A, B>: Mirror2<Constructor = Self, A = A, B = B>;
}

pub trait AppliedTypeConstructor2<A, B>: TypeConstructor2 {
  type Mirror: Mirror2<Constructor = Self, A = A, B = B>; // eventually use an associated type default here.
}

impl<F: TypeConstructor2, A, B> AppliedTypeConstructor2<A, B> for F {
  type Mirror = Self::Of<A, B>;
}

pub trait Mirror2: Sized {
  type Constructor: AppliedTypeConstructor2<Self::A, Self::B, Mirror = Self>;

  type A;
  type B;

  fn as_member(
    self,
  ) -> <Self::Constructor as TypeConstructor2>::Of<Self::A, Self::B> {
    fn witness<F: TypeConstructor2, A, B>(
      fa: <F as AppliedTypeConstructor2<A, B>>::Mirror,
    ) -> F::Of<A, B> {
      fa
    }
    witness::<Self::Constructor, Self::A, Self::B>(self)
  }
}
