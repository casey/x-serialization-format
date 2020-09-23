use crate::common::*;

pub struct I8Serializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for i8 {
  type View = i8;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    serializer.state.write(&self.to_le_bytes());
    serializer.state.continuation()
  }
}

impl View for i8 {
  type Serializer<A: Allocator, C: Continuation<A>> = I8Serializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // All bit patterns of the correct size are valid values of type Self.
    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for I8Serializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    I8Serializer { state }
  }
}

impl FromView for i8 {
  fn from_view(view: &Self::View) -> Self {
    *view
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
