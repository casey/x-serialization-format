use crate::common::*;

pub struct I8Serializer<A: Allocator, C> {
  allocator:    A,
  continuation: PhantomData<C>,
}

impl X for i8 {
  type Serializer<A: Allocator, C: Continuation<A>> = I8Serializer<A, C>;
  type View = i8;
}

impl View for i8 {
  type Native = i8;

  fn to_native(&self) -> Self::Native {
    *self
  }

  fn check<'value>(value: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // This is safe because all bitpatterns of the correct size are valid values of
    // type Self.
    Ok(unsafe { value.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for I8Serializer<A, C> {
  type Native = i8;

  fn new(allocator: A) -> Self {
    I8Serializer {
      continuation: PhantomData,
      allocator,
    }
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    self.allocator.write(&native.borrow().to_le_bytes());
    C::continuation(self.allocator)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[rustfmt::skip]
  fn success() {
    ok( 0i8,     &[0x00]);
    ok( 1i8,     &[0x01]);
    ok(-1i8,     &[0xFF]);
    ok( i8::MAX, &[0x7F]);
    ok( i8::MIN, &[0x80]);
  }
}
