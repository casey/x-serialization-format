use crate::common::*;

pub struct U8Serializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for u8 {
  type Serializer<A: Allocator, C: Continuation<A>> = U8Serializer<A, C>;
  type View = u8;
}

impl View for u8 {
  type Native = u8;

  fn to_native(&self) -> Self::Native {
    *self
  }

  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // This is safe because all bitpatterns of the correct size are valid values of
    // type Self.
    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for U8Serializer<A, C> {
  type Native = u8;

  fn new(state: State<A, C>) -> Self {
    U8Serializer { state }
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    self.state.write(&native.borrow().to_le_bytes());
    self.state.continuation()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[rustfmt::skip]
  fn success() {
    ok( 0u8,     &[0x00]);
    ok( 1u8,     &[0x01]);
    ok( u8::MAX, &[0xFF]);
  }
}
