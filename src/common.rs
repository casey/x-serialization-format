// core
pub(crate) use core::{
  borrow::Borrow,
  convert::TryInto,
  marker::PhantomData,
  mem::{self, MaybeUninit},
};

// traits
pub(crate) use crate::{
  allocator::Allocator, continuation::Continuation, is::Is, serializer::Serializer, view::View,
  x::X,
};

// structs and enums
pub(crate) use crate::{done::Done, error::Error, slice_allocator::SliceAllocator, state::State};

// type aliases
pub(crate) use crate::Result;

#[cfg(feature = "alloc")]
mod alloc {
  // dependencies
  pub(crate) use ::alloc::{collections::TryReserveError, vec::Vec};

  // traits
  pub(crate) use crate::vec_ext::VecExt;

  // structs and enums
  pub(crate) use crate::vec_allocator::VecAllocator;

  #[cfg(test)]
  pub(crate) use ::alloc::vec;
}

#[cfg(feature = "alloc")]
pub(crate) use self::alloc::*;

#[cfg(feature = "std")]
mod std {
  pub(crate) use std::io::{self, Seek, SeekFrom, Write};
}

#[cfg(feature = "std")]
pub(crate) use self::std::*;

#[cfg(test)]
mod test {
  pub(crate) use core::fmt::Debug;

  #[allow(unused)]
  pub(crate) use crate::test::{err, ok};
}

#[cfg(test)]
pub(crate) use self::test::*;
