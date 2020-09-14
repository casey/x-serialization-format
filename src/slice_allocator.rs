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

impl<'slice> Allocator for SliceAllocator<'slice> {
  type Output = &'slice [u8];

  fn write(&mut self, bytes: &[u8]) {
    // TODO: actually return an error here

    for (dst, src) in self.slice[self.offset..].iter_mut().zip(bytes) {
      *dst = *src;
    }
    self.offset += bytes.len();
  }

  fn finish(self) -> Self::Output {
    &self.slice[..self.offset]
  }
}
