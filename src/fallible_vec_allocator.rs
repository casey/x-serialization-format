use crate::common::*;

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

  fn write(&mut self, bytes: &[u8], offset: usize) {
    if self.error.is_some() {
      return;
    }

    // Calculate total number of bytes:
    let end = offset + bytes.len();

    // If total exceeds the length of the vector,
    if end > self.vec.len() {
      // calculate additional number of bytes,
      let additional = end - self.vec.len();

      // and try to extend self.vec by that number of bytes.
      if let Err(error) = self.vec.try_reserve(additional) {
        // If an error occured, drop current allocation to help alleviate memory
        // pressure,
        self.vec = Vec::new();

        // save the error,
        self.error = Some(error);

        // and return. All future writes will be ignored.
        return;
      }
    }

    self.vec.place(bytes, offset);
  }

  fn finish(self, end: usize) -> Self::Output {
    match self.error {
      None => {
        assert_eq!(self.vec.len(), end);
        Ok(self.vec)
      },
      Some(error) => Err(error),
    }
  }
}
