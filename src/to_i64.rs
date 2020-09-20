use crate::common::*;

// Systems with pointers larger than 64 bits may eventually exist, but
// for now let's assume that isize is at most 64 bits, and document that
// assumption with this assert.
const_assert!(mem::size_of::<isize>() <= mem::size_of::<i64>());

pub(crate) trait ToI64 {
  fn to_i64(self) -> i64;
}

impl ToI64 for isize {
  fn to_i64(self) -> i64 {
    #![allow(clippy::as_conversions)]
    self as i64
  }
}
