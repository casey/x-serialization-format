#[repr(C)]
pub struct U16 {
  bytes: [u8; 2],
}
use crate::common::*;

impl X for u16 {
  type Serializer<A: Allocator, C: Continuation<A>> = U16Serializer<A, C>;
  type View = U16;
}

impl From<&U16> for u16 {
  fn from(view: &U16) -> Self {
    view.to_native()
  }
}

impl View for U16 {
  type Native = u16;

  fn to_native(&self) -> Self::Native {
    u16::from_le_bytes(self.bytes)
  }

  fn check<'value>(value: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    Ok(unsafe { value.assume_init_ref() })
  }
}

pub struct U16Serializer<A: Allocator, C>(A, PhantomData<C>);

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for U16Serializer<A, C> {
  type Native = u16;

  fn new(allocator: A) -> Self {
    U16Serializer(allocator, PhantomData)
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    self.0.write(&native.borrow().to_le_bytes());
    C::continuation(self.0)
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  #[test]
  fn serialize() {}
}
