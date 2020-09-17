use crate::common::*;

pub struct State<A: Allocator, C: Continuation<A>> {
  pub(crate) allocator: A,
  pub(crate) state:     C::State,
  continuation:         PhantomData<C>,
}

impl<A: Allocator, C: Continuation<A>> State<A, C> {
  pub fn new(allocator: A, state: C::State) -> Self {
    Self {
      continuation: PhantomData,
      allocator,
      state,
    }
  }

  pub fn continuation(self) -> C {
    C::continuation(self.allocator, self.state)
  }

  pub fn decompose(self) -> (A, C::State) {
    (self.allocator, self.state)
  }

  pub(crate) fn allocator(&mut self) -> &mut A {
    &mut self.allocator
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
  pub fn transform<D: Continuation<A>>(self) -> State<A, D>
  where
    C::State: Is<Type = D::State>,
  {
    State::new(self.allocator, self.state.into_val())
  }
}

// all credit to: https://github.com/clintonmead/is_type
pub trait Is {
  type Type: ?Sized;

  fn into_val(self) -> Self::Type;
}

impl<T> Is for T {
  type Type = T;

  fn into_val(self) -> Self::Type {
    self
  }
}
