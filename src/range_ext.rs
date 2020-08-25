use crate::common::*;

pub(crate) trait RangeExt {
  fn check_value_in_buffer<T>(&self, value: *const T) -> Result<()>;
}

impl RangeExt for Range<*const u8> {
  fn check_value_in_buffer<T>(&self, value: *const T) -> Result<()> {
    let size = mem::size_of::<T>();

    let start = value as *const u8;
    let end = start.wrapping_add(size);

    if end < start {
      return Err(Error::Internal {
        message: "check_value_in_buffer: Value wraps around memory space. This should never \
                  happen.",
      });
    }

    if start < self.start || end > self.end {
      return Err(Error::ValueNotInBuffer {
        buffer: self.clone(),
        value: Range { start, end },
      });
    }

    Ok(())
  }
}
