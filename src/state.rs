use crate::common::*;

pub struct State<A: Allocator, C: Continuation<A>> {
  allocator:    A,
  state:        C::State,
  continuation: PhantomData<C>,
}

impl<A: Allocator, C: Continuation<A>> State<A, C> {
  pub(crate) fn new(allocator: A, state: C::State) -> Self {
    Self {
      continuation: PhantomData,
      allocator,
      state,
    }
  }

  pub(crate) fn continuation(self) -> C {
    C::continuation(self.allocator, self.state)
  }
}
