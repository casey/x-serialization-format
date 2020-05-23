use crate::common::*;

const STORE_IN_ERROR_MESSAGE: &str = "`Offset::store_in`: `Offset` values cannot be copied \
                                      verbatim between buffers. See comment on `Offset::store_in` \
                                      for more details.";

/// A forward-pointing offset that is either null, represented by a zero-valued
/// `size` field, or that points to one or more values of type `T`.
///
/// If not null, the pointer to the first `T` is calculated by taking a pointer
/// to `self`, and adding the value of `size`.
///
/// Because an offset is relative to its own location buffer, moving an offset
/// invalidates it.
#[repr(C)]
#[derive(Debug)]
pub struct Offset<T: View> {
  value: Size,
  _phantom_data: PhantomData<T>,
}

impl<T: View> Offset<T> {
  pub(crate) unsafe fn set(&mut self, value: Size) {
    self.value = value;
  }

  /// Convert this offset to a pointer to T, returning `None` iff this offset is
  /// null.
  pub(crate) fn to_ptr(&self) -> Option<*const T> {
    if self.is_null() {
      None
    } else {
      Some(unsafe { (self as *const Self as *const T).add(self.value.value()) })
    }
  }

  /// True iff this offset is null.
  pub(crate) fn is_null(&self) -> bool {
    self.value.value() == 0
  }

  /// Construct a null offset
  #[cfg(test)]
  pub(crate) fn null() -> Offset<T> {
    Offset {
      value: 0.into(),
      _phantom_data: PhantomData,
    }
  }

  pub(crate) fn check_in_buffer(&self, buffer: Range<*const u8>) -> Result<()> {
    let offset = self as *const Offset<T>;

    buffer.check_value_in_buffer(offset)?;

    Ok(())
  }

  /// Set this offset with the given value. Safe code elsewhere in this crate
  /// relies on the validity of offsets, and can trigger undefined behavior if
  /// an offset does not point the expected number of valid values.
  pub(crate) fn set_from_allocation(
    &mut self,
    buffer: Range<*const u8>,
    allocation: *const T,
  ) -> Result<()> {
    self.check_in_buffer(buffer)?;

    let pointer = self as *const Offset<T> as *const u8;

    let size = (allocation as usize - pointer as usize).into();

    unsafe { self.set(size) };

    Ok(())
  }
}

unsafe impl<T: View> View for Offset<T> {
  /// Offsets don't know how many `T`s they point to. Thus, they have no
  /// additional variable size beyond their own members.
  ///
  /// `View` types that contain offsets must add the total size of each T they
  /// point to to their variable size.
  fn variable_size(&self) -> usize {
    self.value.variable_size()
  }

  /// Checking an offset for validity is somewhat fraught, so this function is
  /// heavily commented.
  fn check(&self, buffer: &[u8]) -> Result<()> {
    // Check the underlying `Size` value
    self.value.check(buffer)?;

    // Convert &self to a pointer
    let pointer = self as *const Self as *const u8;

    // Get the start and end of the buffer
    let range = buffer.try_as_ptr_range()?;

    // Check that the offset is actaully in the given buffer
    self.check_in_buffer(range.clone())?;

    // Get value as a `usize`
    let size = self.value.value();

    // Make sure that pointer + offset does not overflow a usize, and convert it
    // to a usize.
    let offset = if let Some(offset) = (pointer as usize).checked_add(size) {
      offset
    } else {
      return Err(Error::OffsetOverflow { pointer, size });
    };

    // Return an error if this offset points past the end of the buffer
    if offset > range.end as usize {
      return Err(Error::OffsetBounds {
        end: range.end,
        offset,
      });
    }

    Ok(())
  }

  /// An offset cannot be copied verbatim from one buffer into another, thus,
  /// it is almost certainly an error to call `Offset::store_in`. Types that
  /// contain an offset and implement `View::store_in` should pass a mutable
  /// reference to the destination `Offset` to the appropriate `Allocator`
  /// function, which will calculate and store the correct offset to the newly
  /// allocated object.
  fn store_in(&self, _allocation: &mut Self, _allocator: &mut Allocator) -> Result<()> {
    Err(Error::internal(STORE_IN_ERROR_MESSAGE))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn variable_size() {
    assert_eq!(
      Offset::<u8>::null().variable_size(),
      Size::from(0).variable_size()
    );
  }

  #[test]
  fn store_in() {
    let mut allocation = Offset::<u8>::null();
    let mut allocator = Allocator::new(&mut []).unwrap();
    assert_eq!(
      Offset::<u8>::null()
        .store_in(&mut allocation, &mut allocator)
        .unwrap_err(),
      Error::internal(STORE_IN_ERROR_MESSAGE),
    );
  }

  #[test]
  fn null() {
    assert!(Offset::<u8>::null().is_null());
  }

  #[test]
  fn set() {
    let mut offset: Offset<u8> = Offset::null();
    assert!(offset.is_null());

    unsafe { offset.set(1.into()) };

    assert!(!offset.is_null());

    let want = unsafe { ((&offset) as *const Offset<u8> as *const u8).add(1) };

    assert_eq!(offset.to_ptr().unwrap(), want);
  }

  #[test]
  fn zero() {
    let offset = Offset::<u8>::load(&[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    assert!(offset.is_null());
    assert!(offset.to_ptr().is_none());
  }

  #[test]
  fn not_in_buffer() {
    let offset_src = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let offset = Offset::<u8>::load(&offset_src).unwrap();
    let buffer = &[8, 0, 0, 0, 0, 0, 0, 0];
    let error = Error::ValueNotInBuffer {
      buffer: buffer.try_as_ptr_range().unwrap(),
      value: Range {
        start: offset_src.as_ptr() as *const u8,
        end: offset_src
          .as_ptr()
          .wrapping_add(mem::size_of::<Offset<u8>>()),
      },
    };

    assert_eq!(offset.check(buffer).unwrap_err(), error);
  }

  #[test]
  fn overflow() {
    let buffer = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

    let error = Error::OffsetOverflow {
      pointer: buffer.as_ptr(),
      size: 0xFFFFFFFFFFFFFFFF,
    };

    assert_eq!(Offset::<u8>::load(buffer).unwrap_err(), error);
  }

  #[test]
  fn bounds() {
    let buffer = &[16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    Offset::<u8>::load(buffer).unwrap();

    let buffer = &[17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let end = unsafe { buffer.as_ptr().add(buffer.len()) };

    let error = Error::OffsetBounds {
      end,
      offset: end as usize + 1,
    };

    assert_eq!(Offset::<u8>::load(buffer).unwrap_err(), error);
  }

  #[test]
  fn pointer() {
    let buffer = &[8, 0, 0, 0, 0, 0, 0, 0, 0];
    let offset = Offset::<u8>::load(buffer).unwrap();
    let pointer = offset.to_ptr().unwrap();
    assert_eq!(pointer, &buffer[8] as *const u8);
  }
}
