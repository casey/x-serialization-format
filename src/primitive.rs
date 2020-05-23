use crate::common::*;

pub unsafe trait Primitive: Copy {}

unsafe impl<T: Primitive + 'static> View for T {
  fn variable_size(&self) -> usize {
    0
  }

  fn check(&self, _buffer: &[u8]) -> Result<()> {
    Ok(())
  }

  fn store_in(&self, allocation: &mut Self, _allocator: &mut Allocator) -> Result<()> {
    *allocation = *self;
    Ok(())
  }
}
