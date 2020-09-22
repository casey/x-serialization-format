use crate::common::*;

impl X for bool {
  type Serializer<A: Allocator, C: Continuation<A>> = BoolSerializer<A, C>;
  type View = bool;

  fn from_view(view: &Self::View) -> Self {
    *view
  }
}

pub struct BoolSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl View for bool {
  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    assert_eq!(mem::size_of::<bool>(), 1);

    let pointer = suspect.as_ptr() as *const u8;

    // This is safe because the size of a bool is equal to the size of a u8, and
    // we're only reading a u8, and the pointer is non-null because it was derived
    // from a valid reference.
    let value = unsafe { *pointer };

    if value != bool_bit_pattern(true) && value != bool_bit_pattern(false) {
      return Err(Error::Bool { value });
    }

    // This is safe, because the contained bit pattern is that of either true or
    // false, which are valid bools.
    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for BoolSerializer<A, C> {
  type Native = bool;

  fn new(state: State<A, C>) -> Self {
    BoolSerializer { state }
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    let native: bool = *native.borrow();
    // todo: document
    #[allow(clippy::needless_bool)]
    let value = if native { true } else { false };
    let byte = bool_bit_pattern(value);
    self.state.write(&[byte]);
    self.state.continuation()
  }
}

fn bool_bit_pattern(value: bool) -> u8 {
  // This is safe because all bit patterns are valid u8 values
  unsafe { mem::transmute::<bool, u8>(value) }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    ok(false, &[0x00]);
    ok(true, &[0x01]);
  }
}
