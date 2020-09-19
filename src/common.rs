// core
pub(crate) use core::{
  borrow::Borrow,
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
  pub(crate) use ::alloc::{collections::TryReserveError, vec::Vec};

  pub(crate) use crate::vec_allocator::VecAllocator;
}

#[cfg(feature = "alloc")]
pub(crate) use self::alloc::*;

#[cfg(feature = "std")]
mod std {
  pub(crate) use std::io::{self, Seek, Write};
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
