// HKTs with one parameter

pub trait TypeConstructor1 {
  type K<A:'static>: Mirror1<Constructor = Self, A = A>+'static;
}

pub trait AppliedTypeConstructor1<A>: TypeConstructor1 {
  type Mirror: Mirror1<Constructor = Self, A = A>; // eventually use an associated type default here.
}

impl<F: TypeConstructor1, A:'static> AppliedTypeConstructor1<A> for F {
  type Mirror = Self::K<A>;
}

pub trait Mirror1: Sized {
  type A;
  type Constructor: AppliedTypeConstructor1<Self::A, Mirror = Self>;
  fn reify(self) -> <Self::Constructor as TypeConstructor1>::K<Self::A> {
    fn witness<F: TypeConstructor1, A>(
      fa: <F as AppliedTypeConstructor1<A>>::Mirror,
    ) -> F::K<A> {
      fa
    }
    witness::<Self::Constructor, Self::A>(self)
  }
}

// HKTs with two parameters

pub trait TypeConstructor2 {
  type K<A, B>: Mirror2<Constructor = Self, A = A, B = B>;
}

pub trait AppliedTypeConstructor2<A, B>: TypeConstructor2 {
  type Mirror: Mirror2<Constructor = Self, A = A, B = B>; // eventually use an associated type default here.
}

impl<F: TypeConstructor2, A, B> AppliedTypeConstructor2<A, B> for F {
  type Mirror = Self::K<A, B>;
}

pub trait Mirror2: Sized {
  type A;
  type B;
  type Constructor: AppliedTypeConstructor2<Self::A, Self::B, Mirror = Self>;
  fn reify(
    self,
  ) -> <Self::Constructor as TypeConstructor2>::K<Self::A, Self::B> {
    fn witness<F: TypeConstructor2, A, B>(
      fa: <F as AppliedTypeConstructor2<A, B>>::Mirror,
    ) -> F::K<A, B> {
      fa
    }
    witness::<Self::Constructor, Self::A, Self::B>(self)
  }
}
