// use crate::dev::*;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
