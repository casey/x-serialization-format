#![no_std]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

mod allocator;
mod common;
mod continuation;
mod done;
mod serializer;
mod slice_allocator;
mod u16;
mod view;
mod x;

// TODO:
// - test with alloc and not alloc

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod vec_allocator;

// traits
pub use crate::{
  allocator::Allocator, continuation::Continuation, serializer::Serializer, view::View, x::X,
};

// structs and enums
pub use crate::{done::Done, slice_allocator::SliceAllocator};

#[cfg(feature = "alloc")]
pub use crate::vec_allocator::VecAllocator;

#[doc(hidden)]
/// This export is used by `x-derive` to access `core`
pub use core;
