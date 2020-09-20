use crate::common::*;

pub trait View: Sized {
  type Native: X;

  fn to_native(&self) -> Self::Native;

  fn load(buffer: &[u8]) -> Result<&Self> {
    let unchecked = Self::cast(buffer, 0)?;

    let checked = Self::check(unchecked, buffer)?;

    let unchecked_pointer: *const MaybeUninit<Self> = unchecked;
    let checked_pointer: *const Self = checked;

    assert_eq!(checked_pointer as usize, unchecked_pointer as usize);

    Ok(checked)
  }

  fn cast(buffer: &[u8], offset: usize) -> Result<&MaybeUninit<Self>> {
    assert_eq!(
      mem::align_of::<Self>(),
      1,
      "View types must have alignment 1. Alignment is {}.",
      mem::align_of::<Self>(),
    );

    if buffer.len() < offset + mem::size_of::<Self>() {
      return Err(Error::BufferTooSmall);
    }

    // This `add` is safe because the buffer was derived from a valid byte slice
    // containing at least `offset` bytes, which guarantees that a pointer to the
    // first element of the slice plus `offset` will not wrap.
    let pointer = unsafe { buffer.as_ptr().add(offset) } as *const MaybeUninit<Self>;

    // This dereference and reference are safe because:
    // - `pointer` is non-null because it was derived from a valid slice, which must
    //   be non-null.
    // - We have asserted hat the alignment of Self is 1, so there are no alignment
    //   considerations to worry about.
    // - The buffer contained at least as many bytes past the offset as the size of
    //   Self, so all bytes of the value are initialized.
    // - All bit patterns are valid for MaybeUninit<T>.
    Ok(unsafe { &*pointer })
  }

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "View types must have alignment 1. Alignment is 4.")]
  fn alignment_error() {
    #[derive(Debug)]
    struct Foo(u32);

    impl X for Foo {
      type Serializer<A: Allocator, C: Continuation<A>> = Foo;
      type View = Foo;
    }

    impl View for Foo {
      type Native = Foo;

      fn check<'value>(_: &'value MaybeUninit<Self>, _: &[u8]) -> Result<&'value Self> {
        panic!()
      }

      fn to_native(&self) -> Self::Native {
        panic!()
      }
    }

    impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for Foo {
      type Native = Foo;

      fn new(_: State<A, C>) -> Self {
        panic!()
      }

      fn serialize<B: Borrow<Self::Native>>(self, _: B) -> C {
        panic!()
      }
    }

    Foo::load(&[0, 0, 0, 0]).unwrap();
  }
}
