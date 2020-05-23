use crate::common::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Size {
  inner: U64,
}

impl<'a> Value<'a> for Size {
  type Value = usize;

  fn value(&'a self) -> Self::Value {
    self.inner.value().try_into().unwrap()
  }
}

impl From<Size> for usize {
  fn from(x: Size) -> usize {
    x.value()
  }
}

impl From<usize> for Size {
  fn from(x: usize) -> Size {
    // TODO: document and assert safe
    Size {
      inner: (x as u64).into(),
    }
  }
}

unsafe impl View for Size {
  fn variable_size(&self) -> usize {
    self.inner.variable_size()
  }

  fn check(&self, buffer: &[u8]) -> Result<()> {
    self.inner.check(buffer)?;

    let value = self.inner.value();

    let _: usize = value.try_into().map_err(|_| Error::Size { value })?;

    Ok(())
  }

  fn store_in(&self, allocation: &mut Self, allocator: &mut Allocator) -> Result<()> {
    self.inner.store_in(&mut allocation.inner, allocator)?;
    Ok(())
  }
}
