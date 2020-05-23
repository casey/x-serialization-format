use crate::common::*;

unsafe impl Primitive for () {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    test::ok(&[], ());
  }
}
