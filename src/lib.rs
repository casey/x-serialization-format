#![no_std]
#![feature(arbitrary_enum_discriminant)]
#![feature(associated_type_defaults)]
#![feature(concat_idents)]
#![feature(generic_associated_types)]
#![feature(maybe_uninit_ref)]
#![feature(min_const_generics)]
#![feature(raw_ref_op)]
#![feature(try_reserve)]
#![allow(incomplete_features)]

// traits
pub use crate::{
  allocator::Allocator, continuation::Continuation, serializer::Serializer, view::View, x::X,
};

// structs and enums
pub use crate::{done::Done, error::Error, slice_allocator::SliceAllocator, state::State};

// signed inegers
pub use crate::integer::{
  I128Serializer, I16Serializer, I32Serializer, I64Serializer, I128, I16, I32, I64,
};

// unsigned inegers
pub use crate::integer::{
  U128Serializer, U16Serializer, U32Serializer, U64Serializer, U128, U16, U32, U64,
};

#[cfg(feature = "alloc")]
pub use crate::{fallible_vec_allocator::FallibleVecAllocator, vec_allocator::VecAllocator};

#[cfg(feature = "std")]
pub use crate::write_allocator::WriteAllocator;

#[doc(hidden)]
/// This export is used by `x-derive` to access `core`
pub use core;

#[doc(hidden)]
/// This is exported for use in macros in the `derive-x` crate, and is thus not
/// subject to semver compatibility, and may be removed or changed at any time.
pub use is::Is;

// Result type alias with E defaulting to this crates Error type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod allocator;
mod array;
mod bool;
mod common;
mod continuation;
mod done;
mod error;
mod i8;
mod integer;
mod is;
mod serializer;
mod slice_allocator;
mod state;
mod u8;
mod unit;
mod view;
mod x;

#[cfg(feature = "alloc")]
mod vec_allocator;

#[cfg(feature = "alloc")]
mod fallible_vec_allocator;

#[cfg(feature = "alloc")]
mod vec_ext;

#[cfg(feature = "std")]
mod write_allocator;

#[cfg(test)]
mod test;
