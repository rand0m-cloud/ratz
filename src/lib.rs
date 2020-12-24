#![feature(generic_associated_types)]
#![feature(never_type)]

mod associative;
mod associative_both;
mod associative_either;
mod associative_flatten;
mod bifunctor;
mod commutative;
mod covariant;
mod data;
mod derived;
mod hkt;
mod idempotent;
mod identity;
mod identity_both;
mod identity_either;
mod identity_flatten;
mod inverse;
mod profunctor;
mod right_covariant;
mod traversable;

pub mod prelude {
    pub use crate::{
        associative::*,
        associative_both::*,
        associative_either::*,
        associative_flatten::*,
        bifunctor::*,
        commutative::*,
        covariant::*,
        data::{option::*, vec::*},
        derived::{applicative::*, monad::*},
        hkt::*,
        idempotent::*,
        identity::*,
        identity_both::*,
        identity_either::*,
        identity_flatten::*,
        inverse::*,
        profunctor::*,
        right_covariant::*,
        traversable::*,
    };
}

pub mod dev {
    pub use crate::{data::either::*, prelude::*};
}
