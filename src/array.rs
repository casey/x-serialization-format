use crate::common::*;

pub struct ArraySerializer<A: Allocator, C: Continuation<A>, E, const SIZE: usize> {
  allocator:    A,
  serialized:   usize,
  continuation: PhantomData<C>,
  element:      PhantomData<E>,
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
    todo!()
  }
}

impl<A: Allocator, C: Continuation<A>, E: X, const SIZE: usize> Serializer<A, C>
  for ArraySerializer<A, C, E, SIZE>
{
  type Native = [E; SIZE];

  fn new(allocator: A) -> Self {
    ArraySerializer {
      continuation: PhantomData,
      element: PhantomData,
      serialized: 0,
      allocator,
    }
  }

  /// signature is going to be real fucked up
  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    todo!()
  }
}

impl<A: Allocator, C: Continuation<A>, E: X, const SIZE: usize> ArraySerializer<A, C, E, SIZE> {
  fn element_serializer() {}
}

mod foo {
  use crate::common::*;

  pub trait StatefulContinuation<A: Allocator> {
    type State = ();

    fn continuation(allocator: A, state: Self::State) -> Self;
  }
}
