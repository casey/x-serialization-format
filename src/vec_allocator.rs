use crate::common::*;

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

  fn write(&mut self, bytes: &[u8], offset: usize) {
    self.vec.place(bytes, offset);
  }

  fn finish(self, end: usize) -> Self::Output {
    assert_eq!(self.vec.len(), end);
    self.vec
  }
}
