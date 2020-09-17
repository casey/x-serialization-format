use crate::common::*;

struct State<A: Allocator, C: StatefulContinuation<A>> {
  allocator:    A,
  state:        C::State,
  continuation: PhantomData<C>,
}

impl<A: Allocator, C: StatefulContinuation<A>> State<A, C> {
  fn new(allocator: A, state: C::State) -> Self {
    Self {
      continuation: PhantomData,
      allocator,
      state,
    }
  }

  fn continuation(self) -> C {
    C::continuation(self.allocator, self.state)
  }
}
