use crate::common::*;

pub struct U8Serializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for u8 {
  type View = u8;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    serializer.state.write(&self.to_le_bytes());
    serializer.state.continuation()
  }
}

impl FromView for u8 {
  fn from_view(view: &Self::View) -> Self {
    *view
  }
}

impl View for u8 {
  type Serializer<A: Allocator, C: Continuation<A>> = U8Serializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // All bit patterns of the correct size are valid values of type Self.
    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for U8Serializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    U8Serializer { state }
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

  #[test]
  fn unambiguous_load() {
    let buffer: &[u8] = &[10];
    u8::load(&buffer).unwrap();
  }
}
