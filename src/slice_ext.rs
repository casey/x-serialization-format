use crate::common::*;

pub(crate) trait SliceExt<T> {
  /// Return a range where `Range::start` is a pointer to the first element to
  /// the slice, and `Range::end` is a pointer pointing past the end of the last
  /// element in the slice.
  ///
  /// `slice.as_ptr().add(slice.len()` should always be safe, for all valid
  /// slices, however, this is not yet normative, as noted in
  /// [the comments in the unstable slice::as_ptr_range](
  ///   https://doc.rust-lang.org/std/primitive.slice.html#method.as_ptr_range
  /// ).
  ///
  /// So, this version, `try_as_ptr_range` returns a `Result`. When
  /// `slice::as_ptr_range` is stabilized, this can be removed.
  fn try_as_ptr_range(&self) -> Result<Range<*const T>>;
}

impl<'a, T> SliceExt<T> for [T] {
  fn try_as_ptr_range(&self) -> Result<Range<*const T>> {
    let start = self.as_ptr();

    let end = self.as_ptr().wrapping_add(self.len());

    // If `end` is less than `start`, then the above `wrapping_add` has
    // overflowed.
    //
    // This should be impossible for a valid slice. However, the standard
    // library docs don't clearly rule out the possibility, so let's check for
    // it anyways.
    if end < start {
      return Err(Error::internal(
        "Buffer with size greater than `isize::MAX` passed to `SliceExt::try_as_ptr_range`.",
      ));
    }

    Ok(Range { start, end })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn try_as_ptr_range() {
    let slice: &[u8] = &[1, 2, 3];

    let Range { start, end } = slice.try_as_ptr_range().unwrap();

    assert_eq!(start, slice.as_ptr());
    assert_eq!(end, slice.as_ptr().wrapping_add(3));
  }
}
