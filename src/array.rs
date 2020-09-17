use crate::common::*;

pub struct ArraySerializer<A: Allocator, C: Continuation<A>, E, const SIZE: usize> {
  serialized: usize,
  state:      State<A, C>,
  element:    PhantomData<E>,
}

impl<E: X, const SIZE: usize> X for [E; SIZE] {
  type Serializer<A: Allocator, C: Continuation<A>> = ArraySerializer<A, C, E, SIZE>;
  type View = [E::View; SIZE];
}

impl<E: View, const SIZE: usize> View for [E; SIZE] {
  type Native = [E::Native; SIZE];

  fn to_native(&self) -> Self::Native {
    todo!()
  }

  fn check<'value>(value: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // TODO: Actuall implement this!
    Ok(unsafe { value.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>, E: X, const SIZE: usize> Serializer<A, C>
  for ArraySerializer<A, C, E, SIZE>
{
  type Native = [E; SIZE];

  fn new(state: State<A, C>) -> Self {
    ArraySerializer {
      element: PhantomData,
      serialized: 0,
      state,
    }
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    let native = native.borrow();
    for i in 0..SIZE {
      let element_serializer = self.element_serializer();
      self = element_serializer.serialize(&native[i]);
    }
    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, E: X, const SIZE: usize> ArraySerializer<A, C, E, SIZE> {
  pub fn element_serializer(self) -> <E as X>::Serializer<A, ArraySerializer<A, C, E, SIZE>> {
    if self.serialized == SIZE {
      todo!()
    }

    let (allocator, continuation_state) = self.state.decompose();

    let array_state = ArrayState {
      serialized: self.serialized + 1,
      continuation_state,
    };

    let state = State::new(allocator, array_state);

    <E as X>::Serializer::new(state)
  }

  pub fn element<B: Borrow<E>>(self, element: B) -> Self {
    self.element_serializer().serialize(element)
  }

  pub fn done(self) -> C {
    if self.serialized != SIZE {
      todo!()
    }

    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, E: X, const SIZE: usize> Continuation<A>
  for ArraySerializer<A, C, E, SIZE>
{
  type State = ArrayState<A, C>;

  fn continuation(allocator: A, state: Self::State) -> Self {
    ArraySerializer {
      element:    PhantomData,
      serialized: state.serialized,
      state:      State::new(allocator, state.continuation_state),
    }
  }
}

pub struct ArrayState<A: Allocator, C: Continuation<A>> {
  serialized:         usize,
  continuation_state: C::State,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize() {
    type Native = [u8; 2];

    let have = Native::store_to_vec().element(0).element(1).done().done();

    assert_eq!(have, &[0, 1]);
  }
}
