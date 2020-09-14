use crate::common::*;

extern crate alloc;

use alloc::vec::Vec;

pub struct VecAllocator {
  vec: Vec<u8>,
}

impl VecAllocator {
  pub fn new() -> VecAllocator {
    Self { vec: Vec::new() }
  }
}

impl Allocator for VecAllocator {
  type Output = Vec<u8>;

  // TODO: Should this return an alloc result?
  fn write(&mut self, bytes: &[u8]) {
    self.vec.extend(bytes);
  }

  fn finish(self) -> Self::Output {
    self.vec
  }
}
