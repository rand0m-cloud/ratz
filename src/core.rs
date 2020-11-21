use std::marker::PhantomData;
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct OptionFamily;
pub struct VectorFamily;
pub struct ResultFamily<'a, E: 'a> {
    phantom: PhantomData<&'a E>,
}
pub struct ResultFamily2;
