#![no_std]
#![feature(generic_associated_types)]
#![feature(arbitrary_enum_discriminant)]
#![feature(concat_idents)]
#![feature(min_const_generics)]
#![feature(raw_ref_op)]
#![feature(maybe_uninit_ref)]
#![allow(incomplete_features)]

// traits
pub use crate::{
  allocator::Allocator, continuation::Continuation, serializer::Serializer, view::View, x::X,
};

// structs and enums
pub use crate::{done::Done, error::Error, slice_allocator::SliceAllocator};

// signed inegers
pub use crate::integer::{
  I128Serializer, I16Serializer, I32Serializer, I64Serializer, I128, I16, I32, I64,
};

// unsigned inegers
pub use crate::integer::{
  U128Serializer, U16Serializer, U32Serializer, U64Serializer, U128, U16, U32, U64,
};

#[cfg(feature = "alloc")]
pub use crate::vec_allocator::VecAllocator;

#[doc(hidden)]
/// This export is used by `x-derive` to access `core`
pub use core;

// Result type alias with E defaulting to this crates Error type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

mod allocator;
mod common;
mod continuation;
mod done;
mod error;
mod i8;
mod integer;
mod serializer;
mod slice_allocator;
mod u8;
mod unit;
mod view;
mod x;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod vec_allocator;

#[cfg(test)]
mod test;
