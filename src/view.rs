use crate::common::*;

// TODO: Impl X for all View
pub trait View: Sized {
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C>;

  fn to_native<N: X<View = Self> + FromView>(&self) -> N {
    N::from_view(self)
  }

  fn load(buffer: &[u8]) -> Result<&Self> {
    let unchecked = Self::cast(buffer, 0)?;

    let checked = Self::check(unchecked, buffer)?;

    let unchecked_pointer: *const MaybeUninit<Self> = unchecked;
    let checked_pointer: *const Self = checked;

    // TODO: This is required because the `check` implementation could return a
    // reference to a valid value other than the one that was passed in. By doing
    // this assertion, it is guaranteed that the value passed in is valid.
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

// TODO: reenable
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "View types must have alignment 1. Alignment is 4.")]
  fn alignment_error() {
    #[derive(Debug)]
    struct Foo(u32);

    impl X for Foo {
      type View = Foo;

      fn serialize<A: Allocator, C: Continuation<A>>(
        &self,
        serializer: <Self::View as View>::Serializer<A, C>,
      ) -> C {
        todo!()
      }
    }

    impl View for Foo {
      type Serializer<A: Allocator, C: Continuation<A>> = Foo;

      fn check<'value>(_: &'value MaybeUninit<Self>, _: &[u8]) -> Result<&'value Self> {
        panic!()
      }
    }

    impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for Foo {
      fn new(_: State<A, C>) -> Self {
        panic!()
      }
    }

    Foo::load(&[0, 0, 0, 0]).unwrap();
  }
}
