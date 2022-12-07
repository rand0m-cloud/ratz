mod hkt;
mod semigroup;
mod monoid;
mod functor;
mod applicative;
mod monad;
mod foldable;
mod traversable;
mod data;

pub use crate::{
    hkt::*,
    semigroup::Semigroup,
    monoid::Monoid,
    functor::Functor,
    applicative::Applicative,
    monad::Monad,
    foldable::Foldable,
    traversable::Traversable,
};

pub mod syntax {
    pub use crate::{
        functor::FunctorSyntax,
        applicative::ApplicativeSyntax,
        monad::MonadSyntax,
        foldable::FoldableSyntax
    };
}
