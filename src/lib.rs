#![no_std]
#![feature(generic_associated_types)]
#![feature(arbitrary_enum_discriminant)]
#![feature(min_const_generics)]
#![feature(raw_ref_op)]
#![feature(maybe_uninit_ref)]
#![allow(incomplete_features)]

mod allocator;
mod common;
mod continuation;
mod done;
mod error;
mod serializer;
mod slice_allocator;
mod u16;
mod view;
mod x;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod vec_allocator;

// traits
pub use crate::{
  allocator::Allocator, continuation::Continuation, serializer::Serializer, view::View, x::X,
};

// structs and enums
pub use crate::{done::Done, error::Error, slice_allocator::SliceAllocator};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(feature = "alloc")]
pub use crate::vec_allocator::VecAllocator;

#[doc(hidden)]
/// This export is used by `x-derive` to access `core`
pub use core;
