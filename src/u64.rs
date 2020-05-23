use crate::common::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct U64 {
  bytes: [u8; 8],
}

impl From<U64> for u64 {
  fn from(x: U64) -> u64 {
    x.value()
  }
}

impl From<u64> for U64 {
  fn from(x: u64) -> U64 {
    U64 {
      bytes: x.to_le_bytes(),
    }
  }
}

unsafe impl Primitive for U64 {}

impl<'a> Value<'a> for U64 {
  type Value = u64;

  fn value(&'a self) -> Self::Value {
    u64::from_le_bytes(self.bytes)
  }
}
