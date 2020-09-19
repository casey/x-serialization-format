use crate::common::*;

pub struct SliceAllocator<'slice> {
  slice: &'slice mut [u8],
  end:   usize,
}

impl<'slice> SliceAllocator<'slice> {
  pub fn new(slice: &'slice mut [u8]) -> SliceAllocator<'slice> {
    Self { slice, end: 0 }
  }
}

impl<'slice> Allocator for SliceAllocator<'slice> {
  type Output = &'slice [u8];

  fn write(&mut self, bytes: &[u8], offset: usize) {
    for (dst, src) in self.slice[offset..].iter_mut().zip(bytes) {
      *dst = *src;
    }
    self.end = offset + bytes.len();
  }

  fn finish(self) -> Self::Output {
    &self.slice[..self.end]
  }
}

#[cfg(test)]
mod tests {
  // TODO: needs tests
}
