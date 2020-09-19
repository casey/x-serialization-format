use crate::common::*;

pub struct State<A: Allocator, C: Continuation<A>> {
  allocator:    A,
  seed:         C::Seed,
  continuation: PhantomData<C>,
}

impl<A: Allocator, C: Continuation<A>> State<A, C> {
  pub fn new(allocator: A, seed: C::Seed) -> Self {
    Self {
      continuation: PhantomData,
      allocator,
      seed,
    }
  }

  pub fn continuation(self) -> C {
    C::continuation(self.allocator, self.seed)
  }

  pub fn decompose(self) -> (A, C::Seed) {
    (self.allocator, self.seed)
  }

  pub(crate) fn write(&mut self, bytes: &[u8]) {
    self.allocator.write(bytes);
  }

  /// Transform this state into the state for another continuation.
  ///
  /// Only implemented if the type of the current continuation's state,
  /// C::State, is that same as that of the destination continuation's state,
  /// D::State.
  ///
  /// This is useful when state for a sub-object is needed, and the parent
  /// object has no state of its own. This occurs when serializing composite
  /// types where all state is lifted into the type system, and thus no run-time
  /// state exists.
  pub fn identity<D: Continuation<A>>(self) -> State<A, D>
  where
    C::Seed: Is<Type = D::Seed>,
  {
    State::new(self.allocator, self.seed.identity())
  }
}

// write(bytes: &[u8], offset: usize);
// but also needs to be able to reserve space for a new object,
// and get its offset
