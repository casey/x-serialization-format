use crate::common::*;

use alloc::vec::Vec;

#[derive(Default)]
pub struct FallibleVecAllocator {
  vec:   Vec<u8>,
  error: Option<TryReserveError>,
}

impl FallibleVecAllocator {
  pub fn new() -> FallibleVecAllocator {
    FallibleVecAllocator::default()
  }
}

impl Allocator for FallibleVecAllocator {
  type Output = Result<Vec<u8>, TryReserveError>;

  fn write(&mut self, bytes: &[u8]) {
    if self.error.is_some() {
      return;
    }

    if let Err(error) = self.vec.try_reserve(bytes.len()) {
      self.error = Some(error);
      return;
    }

    self.vec.extend(bytes);
  }

  fn finish(self) -> Self::Output {
    match self.error {
      None => Ok(self.vec),
      Some(error) => Err(error),
    }
  }
}
