pub(crate) use core::{
  borrow::Borrow,
  marker::PhantomData,
  mem::{self, MaybeUninit},
};

pub(crate) use crate::{
  allocator::Allocator, continuation::Continuation, serializer::Serializer, view::View, x::X,
};

pub(crate) use crate::{done::Done, slice_allocator::SliceAllocator};

// type aliases
pub(crate) use crate::Result;

#[cfg(feature = "alloc")]
mod alloc {
  pub(crate) use ::alloc::vec::Vec;

  pub(crate) use crate::vec_allocator::VecAllocator;
}

#[cfg(feature = "alloc")]
pub(crate) use self::alloc::*;
