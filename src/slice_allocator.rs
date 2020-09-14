use crate::common::*;

pub struct SliceAllocator<'slice> {
  slice:  &'slice mut [u8],
  offset: usize,
}

impl<'slice> SliceAllocator<'slice> {
  pub fn new(slice: &'slice mut [u8]) -> SliceAllocator<'slice> {
    Self { slice, offset: 0 }
  }
}

impl<'a> Allocator for SliceAllocator<'a> {
  fn write(&mut self, bytes: &[u8]) {
    for (dst, src) in self.slice[self.offset..].iter_mut().zip(bytes) {
      *dst = *src;
    }
    self.offset += bytes.len();
  }
}
