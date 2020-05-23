use crate::common::*;

unsafe impl Primitive for u8 {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    test::ok(&[0], 0u8);
    test::ok(&[1], 1u8);
    test::ok(&[255], 255u8);
    test::err::<u8>(&[], Error::Bounds { over: 1 });
  }
}
