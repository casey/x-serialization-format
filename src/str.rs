use crate::common::*;

#[repr(C)]
pub struct Str {
  bytes: Slice<u8>,
}

impl AsRef<str> for Str {
  fn as_ref(&self) -> &str {
    unsafe { str::from_utf8_unchecked(self.bytes.as_ref()) }
  }
}

impl<'a> From<&'a Str> for &'a str {
  fn from(x: &'a Str) -> &'a str {
    x.as_ref()
  }
}

unsafe impl View for Str {
  fn variable_size(&self) -> usize {
    self.bytes.variable_size()
  }

  fn check(&self, buffer: &[u8]) -> Result<()> {
    self.bytes.check(buffer)?;

    let bytes = self.bytes.as_ref();

    str::from_utf8(bytes).map_err(|source| Error::StringDecode { source })?;

    Ok(())
  }

  fn store_in(&self, allocation: &mut Self, allocator: &mut Allocator) -> Result<()> {
    self.bytes.store_in(&mut allocation.bytes, allocator)?;
    Ok(())
  }
}
