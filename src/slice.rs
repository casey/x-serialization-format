use crate::common::*;

#[repr(C)]
pub struct Slice<T: View> {
  offset: Offset<T>,
  len: Size,
}

impl<T: View> AsRef<[T]> for Slice<T> {
  fn as_ref(&self) -> &[T] {
    self.value()
  }
}

impl<'a, T: View> Value<'a> for Slice<T> {
  type Value = &'a [T];

  fn value(&'a self) -> Self::Value {
    unsafe { core::slice::from_raw_parts::<T>(self.offset.to_ptr().unwrap(), self.len.value()) }
  }
}

impl<'a, T: View> IntoIterator for &'a Slice<T> {
  type IntoIter = slice::Iter<'a, T>;
  type Item = &'a T;

  fn into_iter(self) -> Self::IntoIter {
    self.as_ref().into_iter()
  }
}

unsafe impl<T: View> View for Slice<T> {
  fn variable_size(&self) -> usize {
    self.offset.variable_size()
      + self.len.variable_size()
      + self
        .value()
        .iter()
        .map(|element| element.variable_size())
        .sum::<usize>()
  }

  fn check(&self, buffer: &[u8]) -> Result<()> {
    self.offset.check(buffer)?;
    self.len.check(buffer)?;

    // TODO: replace offset with reference

    let buffer_start = buffer.as_ptr();
    let buffer_end = unsafe { buffer_start.add(buffer.len()) };

    let start = self.offset.to_ptr().unwrap() as *const u8;

    let len = self.len.value();

    let element_size = T::FIXED_SIZE;

    let bytes = len
      .checked_mul(element_size)
      .ok_or_else(|| Error::SliceLenOverflow { len, element_size })?;

    (start as usize)
      .checked_add(bytes)
      .ok_or_else(|| Error::SliceEndOverflow { start, bytes })?;

    let end = unsafe { start.add(bytes) };

    if end > buffer_end {
      return Err(Error::SliceBounds { end, buffer_end });
    }

    for element in self {
      element.check(buffer)?;
    }

    Ok(())
  }

  fn store_in(&self, allocation: &mut Self, allocator: &mut Allocator) -> Result<()> {
    self.len.store_in(&mut allocation.len, allocator)?;

    allocator.store_slice(&mut allocation.offset, self.as_ref())?;

    Ok(())
  }
}
