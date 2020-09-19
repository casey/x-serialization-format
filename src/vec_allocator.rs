use crate::common::*;

use alloc::vec::Vec;

#[derive(Default)]
pub struct VecAllocator {
  vec: Vec<u8>,
}

impl VecAllocator {
  pub fn new() -> VecAllocator {
    VecAllocator::default()
  }
}

impl Allocator for VecAllocator {
  type Output = Vec<u8>;

  fn write(&mut self, bytes: &[u8]) {
    self.vec.extend(bytes);
  }

  fn finish(self) -> Self::Output {
    self.vec
  }
}
