use crate::common::*;

pub struct ArraySerializer<A: Allocator, C: Continuation<A>, E, const SIZE: usize> {
  serialized: usize,
  state:      State<A, C>,
  element:    PhantomData<E>,
}

impl<E: X, const SIZE: usize> X for [E; SIZE] {
  type View = [E::View; SIZE];

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    for element in self {
      serializer = serializer.element_serializer::<E>().serialize(element);
    }

    serializer.done()
  }
}

impl<E: View, const SIZE: usize> View for [E; SIZE] {
  type Serializer<A: Allocator, C: Continuation<A>> = ArraySerializer<A, C, E, SIZE>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let pointer: *const [E; SIZE] = suspect.as_ptr();

    let pointer = pointer as *const [MaybeUninit<E>; SIZE];

    // TODO: is this safe?
    let suspects = unsafe { &*pointer };

    for suspect_element in suspects {
      View::check(suspect_element, buffer)?;
    }

    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>, E: View, const SIZE: usize> Serializer<A, C>
  for ArraySerializer<A, C, E, SIZE>
{
  fn new(state: State<A, C>) -> Self {
    ArraySerializer {
      element: PhantomData,
      serialized: 0,
      state,
    }
  }
}

impl<A: Allocator, C: Continuation<A>, E: View, const SIZE: usize> ArraySerializer<A, C, E, SIZE> {
  pub fn element_serializer<N: X<View = E>>(
    self,
  ) -> <N::View as View>::Serializer<A, ArraySerializer<A, C, E, SIZE>> {
    if self.serialized == SIZE {
      todo!()
    }

    let serialized = self.serialized;

    let state = self
      .state
      .transform(|inner| ArraySeed { serialized, inner });

    <N::View as View>::Serializer::new(state)
  }

  pub fn element<N: X<View = E>>(self, element: N) -> Self {
    self.element_serializer::<N>().serialize(&element)
  }

  pub fn done(self) -> C {
    if self.serialized != SIZE {
      todo!()
    }

    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, E: View, const SIZE: usize> Continuation<A>
  for ArraySerializer<A, C, E, SIZE>
{
  type Seed = ArraySeed<A, C>;

  fn continuation(state: State<A, Self>) -> Self {
    let serialized = state.seed().serialized + 1;

    ArraySerializer {
      element: PhantomData,
      state: state.transform(|seed| seed.inner),
      serialized,
    }
  }
}

pub struct ArraySeed<A: Allocator, C: Continuation<A>> {
  serialized: usize,
  inner:      C::Seed,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize() {
    type Native = [u8; 2];

    // TODO: It sucks that I now have to use type inference. Can I have a default
    // type to avoid this?
    let have = Native::store_to_vec()
      .element(0u8)
      .element(1u8)
      .done()
      .done();

    assert_eq!(have, &[0, 1]);
  }

  #[test]
  #[ignore]
  fn error() {
    todo!()
  }
}
