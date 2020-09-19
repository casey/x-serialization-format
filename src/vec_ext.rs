use crate::common::*;

pub(crate) trait VecExt {
  fn place(&mut self, bytes: &[u8], offset: usize);
}

impl VecExt for Vec<u8> {
  fn place(&mut self, bytes: &[u8], offset: usize) {
    // If self.vec is shorter than the requested offset,
    if self.len() < offset {
      // fill vec with zeros up to offset.
      self.resize_with(offset, Default::default);
    }

    // We can directly set this many bytes:
    let settable = self.len().saturating_sub(offset).min(bytes.len());

    // Set bytes:
    for i in 0..settable {
      self[offset + i] = bytes[i];
    }

    // Any bytes left over should be passed to extend:
    self.extend_from_slice(&bytes[settable..]);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty() {
    let mut vec = Vec::new();

    vec.place(&[1, 2, 3], 0);

    assert_eq!(vec, &[1, 2, 3]);
  }

  #[test]
  fn extend() {
    let mut vec = vec![0, 1, 2];

    vec.place(&[3, 4, 5], 3);

    assert_eq!(vec, &[0, 1, 2, 3, 4, 5]);
  }

  #[test]
  fn extend_after_gap() {
    let mut vec = vec![0, 1, 2];

    vec.place(&[6, 7, 8], 6);

    assert_eq!(vec, &[0, 1, 2, 0, 0, 0, 6, 7, 8]);
  }

  #[test]
  fn set_and_extend() {
    let mut vec = vec![0, 1, 2, 9, 9, 9];

    vec.place(&[3, 4, 5, 6, 7, 8], 3);

    assert_eq!(vec, &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
  }
}
