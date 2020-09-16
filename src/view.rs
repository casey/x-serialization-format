use crate::common::*;

pub trait View: Sized {
  type Native: X;

  fn to_native(&self) -> Self::Native;

  fn load(buffer: &[u8]) -> Result<&Self> {
    let unchecked = Self::cast(buffer, 0)?;

    let checked = Self::check(unchecked, buffer)?;

    Ok(checked)
  }

  fn cast(buffer: &[u8], offset: usize) -> Result<&MaybeUninit<Self>> {
    assert_eq!(mem::align_of::<Self>(), 1);

    if buffer.len() < offset + mem::size_of::<Self>() {
      todo!();
    }

    let pointer = unsafe { buffer.as_ptr().add(offset) } as *const MaybeUninit<Self>;

    Ok(unsafe { &*pointer })
  }

  fn check<'value>(value: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self>;
}
