use crate::common::*;

#[repr(C)]
#[derive(Debug)]
pub struct Offset<V: View> {
  inner: Usize,
  view:  PhantomData<V>,
}

impl<V: View> Offset<V> {
  pub(crate) fn as_ptr(&self) -> *const V {
    let base: *const Self = self;
    let base = base as *const u8;
    // TODO: justify
    let start = unsafe { base.add(self.inner.to_native()) };
    start as *const V
  }

  /// Check that `suspect` is a valid offset that points to `length` valid
  /// elements.
  pub(crate) fn check<'value>(
    suspect: &'value MaybeUninit<Self>,
    buffer: &[u8],
    length: usize,
  ) -> Result<&'value [V]> {
    let buffer_range = Range {
      start: buffer.as_ptr(),
      end:   buffer.as_ptr().wrapping_add(buffer.len()),
    };

    // Check that offset itself is contained within the buffer:
    {
      let offset = Range {
        start: suspect.as_ptr() as *const u8,
        end:   (suspect.as_ptr() as *const u8).wrapping_add(mem::size_of::<Self>()),
      };

      if !buffer_range.contains_range(&offset) {
        return Err(Error::OffsetBounds {
          buffer: buffer_range,
          offset,
        });
      }
    }

    let inner = suspect.cast::<Usize>();

    let inner = View::check(inner, buffer)?;

    let offset = inner.to_native();

    if offset == 0 {
      return Err(Error::OffsetNull);
    }

    if offset < mem::size_of::<Self>() {
      return Err(Error::OffsetValue { value: offset });
    }

    let value = unsafe { suspect.assume_init_ref() };

    let start = value.as_ptr();

    // Check that elements are within range:
    {
      let start = start as *const u8;

      let end = start.wrapping_add(length * mem::size_of::<V>());

      if end < start {
        return Err(Error::OffsetWrap { start, end });
      }

      let element_range = Range { start, end };

      if !buffer_range.contains_range(&element_range) {
        return Err(Error::OffsetElementBounds {
          buffer:   buffer_range,
          elements: element_range,
        });
      }
    }

    let slice: &[MaybeUninit<V>] =
      unsafe { slice::from_raw_parts(start as *const MaybeUninit<V>, length) };

    for element in slice {
      View::check(element, buffer)?;
    }

    Ok(value.to_slice(length))
  }

  fn to_slice(&self, length: usize) -> &[V] {
    unsafe { slice::from_raw_parts(self.as_ptr(), length) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn error_not_in_buffer() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    assert_eq!(
      Offset::check(offset, &buffer[8..16], 0).unwrap_err(),
      Error::OffsetBounds {
        offset: Range {
          start: buffer.as_ptr(),
          end:   buffer.as_ptr().wrapping_add(8),
        },
        buffer: Range {
          start: buffer.as_ptr().wrapping_add(8),
          end:   buffer.as_ptr().wrapping_add(16),
        },
      },
    );
  }

  #[test]
  fn error_null() {
    let buffer: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    assert_eq!(
      Offset::check(offset, buffer, 0).unwrap_err(),
      Error::OffsetNull,
    );
  }

  #[test]
  fn error_value() {
    let buffer: &[u8] = &[7, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    assert_eq!(
      Offset::check(offset, buffer, 0).unwrap_err(),
      Error::OffsetValue { value: 7 }
    );
  }

  #[test]
  fn error_wrap() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    assert_eq!(
      Offset::check(offset, buffer, usize::MAX - 8).unwrap_err(),
      Error::OffsetWrap {
        start: buffer.as_ptr().wrapping_add(8),
        end:   buffer.as_ptr().wrapping_add(usize::MAX),
      }
    );
  }

  #[test]
  fn error_bounds_both_past_end_of_buffer() {
    let buffer: &[u8] = &[9, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    assert_eq!(
      Offset::check(offset, buffer, 0).unwrap_err(),
      Error::OffsetElementBounds {
        buffer:   Range {
          start: buffer.as_ptr(),
          end:   buffer.as_ptr().wrapping_add(buffer.len()),
        },
        elements: Range {
          start: buffer.as_ptr().wrapping_add(9),
          end:   buffer.as_ptr().wrapping_add(9),
        },
      }
    );
  }

  #[test]
  fn error_bounds_end_past_end_of_buffer() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    assert_eq!(
      Offset::check(offset, buffer, 1).unwrap_err(),
      Error::OffsetElementBounds {
        buffer:   Range {
          start: buffer.as_ptr(),
          end:   buffer.as_ptr().wrapping_add(buffer.len()),
        },
        elements: Range {
          start: buffer.as_ptr().wrapping_add(8),
          end:   buffer.as_ptr().wrapping_add(9),
        },
      }
    );
  }

  #[test]
  fn error_invalid_first_element() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<bool>>) };

    assert_eq!(Offset::check(offset, buffer, 3).unwrap_err(), Error::Bool {
      value: 2,
    },);
  }

  #[test]
  fn error_invalid_middle_element() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<bool>>) };

    assert_eq!(Offset::check(offset, buffer, 3).unwrap_err(), Error::Bool {
      value: 2,
    },);
  }

  #[test]
  fn error_invalid_last_element() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<bool>>) };

    assert_eq!(Offset::check(offset, buffer, 3).unwrap_err(), Error::Bool {
      value: 2,
    },);
  }

  #[test]
  fn ok_end() {
    let buffer: &[u8] = &[8, 0, 0, 0, 0, 0, 0, 0];

    let offset = unsafe { &*(buffer.as_ptr() as *const MaybeUninit<Offset<u8>>) };

    Offset::check(offset, buffer, 0).unwrap();
  }
}
