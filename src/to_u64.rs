use crate::common::*;

// Systems with pointers larger than 64 bits may eventually exist, but
// for now let's assume that usize is at most 64 bits, and document that
// assumption with this assert.
const_assert!(mem::size_of::<usize>() <= mem::size_of::<u64>());

pub(crate) trait ToU64 {
  fn to_u64(self) -> u64;
}

impl ToU64 for usize {
  fn to_u64(self) -> u64 {
    #![allow(clippy::as_conversions)]
    self as u64
  }
}
