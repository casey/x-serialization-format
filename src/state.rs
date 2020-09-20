use crate::common::*;

pub struct State<A: Allocator, C: Continuation<A>> {
  allocator:    A,
  seed:         C::Seed,
  continuation: PhantomData<C>,
  end:          usize,
  // TODO: This can't be a vec in no-alloc/no-std contexts
  stack:        Vec<usize>,
}

impl<A: Allocator, C: Continuation<A>> State<A, C> {
  pub fn new(allocator: A, seed: C::Seed) -> Self {
    Self {
      continuation: PhantomData,
      end: 0,
      stack: Vec::new(),
      allocator,
      seed,
    }
  }

  pub fn continuation(self) -> C {
    C::continuation(self)
  }

  /// Transform this state into the state for another continuation by applying a
  /// function to the seed.
  pub fn transform<D: Continuation<A>, W: Fn(C::Seed) -> D::Seed>(
    self,
    transformer: W,
  ) -> State<A, D> {
    State {
      allocator:    self.allocator,
      end:          self.end,
      stack:        self.stack,
      seed:         transformer(self.seed),
      continuation: PhantomData,
    }
  }

  pub(crate) fn end(&self) -> usize {
    self.end
  }

  pub(crate) fn push(&mut self, size: usize) {
    self.stack.push(self.end);
    self.end += size;
  }

  pub(crate) fn pop(&mut self) {
    self.stack.pop().unwrap();
  }

  pub(crate) fn write(&mut self, bytes: &[u8]) {
    if self.stack.is_empty() {
      panic!("State::write: Empty stack.");
    }
    self.allocator.write(bytes, self.stack[0]);
    self.stack[0] += bytes.len();
  }

  pub(crate) fn finish(mut self) -> A::Output {
    self.pop();
    assert_eq!(self.stack.len(), 0);
    self.allocator.finish(self.end)
  }

  pub(crate) fn seed(&self) -> &C::Seed {
    &self.seed
  }

  /// Transform this state into the state for another continuation by reusing
  /// the current seed.
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
    self.transform(Is::identity)
  }
}
