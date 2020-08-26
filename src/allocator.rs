use crate::common::*;

#[derive(Debug, PartialEq)]
pub struct Allocator<'a> {
  range: Range<*const u8>,
  free: &'a mut [u8],
}

impl<'a> Allocator<'a> {
  pub(crate) fn new(buffer: &mut [u8]) -> Result<Allocator> {
    let range = {
      let const_buffer: &[u8] = buffer;
      const_buffer.try_as_ptr_range()?
    };
    Ok(Allocator {
      free: buffer,
      range,
    })
  }

  fn split_off(&mut self, size: usize) -> &'a mut [u8] {
    let free = mem::replace(&mut self.free, &mut []);
    let (allocation, free) = free.split_at_mut(size);
    self.free = free;
    allocation
  }

  fn allocate<T: View>(&mut self, len: usize) -> Result<&'a mut [T]> {
    let have = self.free.len();
    let want = len * T::FIXED_SIZE;
    if have < want {
      return Err(Error::Allocate { have, want });
    }

    let bytes = self.split_off(want);

    let data = bytes.as_mut_ptr() as *mut T;

    Ok(unsafe { core::slice::from_raw_parts_mut(data, len) })
  }

  pub(crate) fn store<T: View>(&mut self, data: &T) -> Result<()> {
    let allocation = self.allocate::<T>(1)?;
    data.store_in(&mut allocation[0], self)?;
    Ok(())
  }

  pub(crate) fn store_slice<T: View>(&mut self, offset: &mut Offset<T>, slice: &[T]) -> Result<()> {
    let allocation = self.allocate::<T>(slice.len())?;

    for (src, dst) in slice.iter().zip(allocation.iter_mut()) {
      src.store_in(dst, self)?;
    }

    self.set_offset(offset, allocation)?;

    Ok(())
  }

  fn set_offset<T: View>(&self, offset: &mut Offset<T>, allocation: &[T]) -> Result<()> {
    offset.set_from_allocation(self.range.clone(), allocation.as_ptr())?;
    Ok(())
  }

  #[cfg(test)]
  fn free(&self) -> usize {
    self.free.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let buffer = &mut [1, 2, 3];
    let range = buffer.as_ref().try_as_ptr_range().unwrap();
    let allocator = Allocator::new(buffer).unwrap();

    assert_eq!(
      allocator,
      Allocator {
        free: &mut [1, 2, 3],
        range,
      }
    )
  }

  #[test]
  fn allocate() {
    let buffer = &mut [0; 32];
    let mut allocator = Allocator::new(buffer).unwrap();

    assert_eq!(allocator.free(), 32);

    allocator.allocate::<u8>(1).unwrap();

    assert_eq!(allocator.free(), 31);

    allocator.allocate::<U64>(1).unwrap();

    assert_eq!(allocator.free(), 23);

    allocator.allocate::<U64>(2).unwrap();

    assert_eq!(allocator.free(), 7);

    assert_eq!(
      allocator.allocate::<U64>(2).unwrap_err(),
      Error::Allocate { have: 7, want: 16 }
    );

    allocator.allocate::<u8>(7).unwrap();

    assert_eq!(allocator.free(), 0);

    assert_eq!(
      allocator.allocate::<u8>(20).unwrap_err(),
      Error::Allocate { have: 0, want: 20 }
    );
  }

  #[test]
  #[rustfmt::skip]
  fn store() {
    let buffer = &mut [0; 32];
    let mut allocator = Allocator::new(buffer).unwrap();

    let zero = U64::from(0);

    let one = U64::from(u64::MAX);

    allocator.store(&zero).unwrap();
    allocator.store(&one).unwrap();
    allocator.store(&zero).unwrap();
    allocator.store(&one).unwrap();

    assert_eq!(
      buffer,
      &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      ]
    )
  }

  #[test]
  fn bad_offset() {
    let buffer = &mut [0, 0, 0, 0];
    let mut offset = Offset::null();

    let start = &offset as *const Offset<u8> as *const u8;
    let end = start.wrapping_add(mem::size_of::<Offset<u8>>());

    let error = Error::ValueNotInBuffer {
      buffer: buffer.try_as_ptr_range().unwrap(),
      value: Range { start, end },
    };

    let mut allocator = Allocator::new(buffer).unwrap();

    assert_eq!(
      allocator
        .store_slice::<u8>(&mut offset, &[1, 2, 3, 4])
        .unwrap_err(),
      error,
    );
  }

  #[test]
  #[rustfmt::skip]
  fn store_slice() {
    let buffer: &mut [u8] = &mut [0; 48];
    let mut allocator = Allocator::new(buffer).unwrap();

    let zero = U64::from(0);

    let one = U64::from(u64::MAX);

    let offsets = allocator.allocate::<Offset<U64>>(2).unwrap();

    allocator.store_slice(&mut offsets[0], &[zero, one]).unwrap();
    allocator.store_slice(&mut offsets[1], &[one, zero]).unwrap();

    // After allocating, buffer should contain:
    // +-----+----------+---------------------------------------------+
    // |0x00 | offset 0 | points to 0x10, value is 0x10 - 0x00 = 0x10 |
    // |0x08 | offset 1 | points to 0x20, value is 0x20 - 0x08 = 0x18 |
    // |0x10 | zero     | all zeros                                   |
    // |0x18 | one      | all ones                                    |
    // |0x20 | one      | all ones                                    |
    // |0x28 | zero     | all zeros                                   |
    // +-----+----------+---------------------------------------------+

    let want: &[u8] = &[
      0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    assert_eq!(buffer, want);
  }
}
