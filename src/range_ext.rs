use crate::common::*;

pub(crate) trait RangeExt<Idx: PartialOrd> {
  fn contains_range(&self, other: &Range<Idx>) -> bool;
}

impl<Idx: PartialOrd> RangeExt<Idx> for Range<Idx> {
  fn contains_range(&self, other: &Range<Idx>) -> bool {
    self.start <= other.start && self.end >= other.end
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    assert!(!(0..0).contains_range(&(1..1)));
    assert!(!(1..1).contains_range(&(0..0)));
    assert!(!(0..1).contains_range(&(1..2)));
    assert!(!(1..2).contains_range(&(0..1)));
    assert!(!(0..2).contains_range(&(1..3)));
    assert!(!(1..3).contains_range(&(0..2)));
    assert!((0..0).contains_range(&(0..0)));
    assert!((0..1).contains_range(&(0..1)));
    assert!((1..1).contains_range(&(1..1)));
  }
}
