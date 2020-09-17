use crate::common::*;

impl X for bool {
  type Serializer<A: Allocator, C: Continuation<A>> = BoolSerializer<A, C>;
  type View = bool;
}

pub struct BoolSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl View for bool {
  type Native = bool;

  fn to_native(&self) -> Self::Native {
    *self
  }

  fn check<'value>(value: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    assert_eq!(mem::size_of::<bool>(), 1);

    let pointer = value.as_ptr() as *const u8;

    // This is safe because the size of a bool is equal to the size of a u8, and
    // we're only reading a u8.
    let byte = unsafe { *pointer };

    // These are safe because all bit patterns are valid for u8.
    let true_byte = unsafe { mem::transmute::<bool, u8>(true) };
    let false_byte = unsafe { mem::transmute::<bool, u8>(false) };

    if byte != true_byte && byte != false_byte {
      panic!();
    }

    // This is safe, because the contained bitpattern is that of either true or
    // false, which are valid bools.
    Ok(unsafe { value.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for BoolSerializer<A, C> {
  type Native = bool;

  fn new(state: State<A, C>) -> Self {
    BoolSerializer { state }
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    let native: bool = *native.borrow();
    let value = unsafe { mem::transmute::<bool, u8>(native) };
    self.state.allocator().write(&[value]);
    self.state.continuation()
  }
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
