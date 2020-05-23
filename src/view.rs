use crate::common::*;

pub unsafe trait View: Sized + 'static {
  const FIXED_SIZE: usize = mem::size_of::<Self>();

  fn total_size(&self) -> usize {
    Self::FIXED_SIZE + self.variable_size()
  }

  fn load(buffer: &[u8]) -> Result<&Self, Error> {
    let alignment = mem::align_of::<Self>();

    if alignment != 1 {
      return Err(Error::Alignment { alignment });
    }

    let end = Self::FIXED_SIZE;

    if end > buffer.len() {
      return Err(Error::Bounds {
        over: end - buffer.len(),
      });
    }

    let reference = unsafe { &*(buffer.as_ptr() as *const Self) };

    reference.check(buffer)?;

    Ok(reference)
  }

  fn store(&self, buffer: &mut [u8]) -> Result<()> {
    let total_size = self.total_size();

    if buffer.len() < total_size {
      return Err(Error::Space {
        buffer_size: buffer.len(),
        total_size,
      });
    }

    let mut allocator = Allocator::new(buffer)?;

    allocator.store(self)?;

    Ok(())
  }

  fn variable_size(&self) -> usize;

  fn check(&self, buffer: &[u8]) -> Result<()>;

  fn store_in(&self, allocation: &mut Self, allocator: &mut Allocator) -> Result<()>;
}
