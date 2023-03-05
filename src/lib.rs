mod applicative;
mod data;
mod foldable;
mod functor;
mod hkt;
mod monad;
mod monoid;
mod semigroup;
mod traversable;

pub use crate::applicative::Applicative;
pub use crate::foldable::Foldable;
pub use crate::functor::Functor;
pub use crate::hkt::*;
pub use crate::monad::Monad;
pub use crate::monoid::Monoid;
pub use crate::semigroup::Semigroup;
pub use crate::traversable::Traversable;

pub mod syntax {
  pub use crate::applicative::ApplicativeSyntax;
  pub use crate::foldable::FoldableSyntax;
  pub use crate::functor::FunctorSyntax;
  pub use crate::monad::MonadSyntax;
}
