use crate::common::*;

pub struct I8Serializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for i8 {
  type Serializer<A: Allocator, C: Continuation<A>> = I8Serializer<A, C>;
  type View = i8;

  fn from_view(view: &Self::View) -> Self {
    *view
  }
}

impl View for i8 {
  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // All bit patterns of the correct size are valid values of type Self.
    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for I8Serializer<A, C> {
  type Native = i8;

  fn new(state: State<A, C>) -> Self {
    I8Serializer { state }
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
    ok( 0i8,     &[0x00]);
    ok( 1i8,     &[0x01]);
    ok(-1i8,     &[0xFF]);
    ok( i8::MAX, &[0x7F]);
    ok( i8::MIN, &[0x80]);
  }
}
