// core
pub(crate) use core::{
  convert::TryInto,
  fmt::Debug,
  marker::PhantomData,
  mem,
  ops::Range,
  slice,
  str::{self, Utf8Error},
};

// type defs
pub(crate) use crate::result::Result;

// structs and enums
pub(crate) use crate::{
  allocator::Allocator, error::Error, offset::Offset, size::Size, slice::Slice, u64::U64,
};

// traits
pub(crate) use crate::{
  primitive::Primitive, range_ext::RangeExt, slice_ext::SliceExt, value::Value, view::View,
};

// imports for tests
#[cfg(test)]
mod testing {
  // modules
  pub(crate) use crate::test;
}

// expose imports from `testing` module
#[cfg(test)]
pub(crate) use testing::*;
