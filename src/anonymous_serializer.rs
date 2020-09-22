#![allow(unused)]

use crate::common::*;

pub(crate) struct AnonymousSerializer<A: Allocator, C: Continuation<A>, T> {
  state: State<A, C>,
  types: PhantomData<T>,
}

impl<A: Allocator, C: Continuation<A>, T> AnonymousSerializer<A, C, T> {
  pub(crate) fn new(state: State<A, C>) -> Self {
    Self {
      types: PhantomData,
      state,
    }
  }
}

impl<A: Allocator, C: Continuation<A>, F: X, R> AnonymousSerializer<A, C, (F, R)> {
  pub(crate) fn serialize<B: Borrow<F>>(self, borrow: B) -> AnonymousSerializer<A, C, R> {
    <F as X>::Serializer::new(self.state.identity()).serialize(borrow)
  }
}

impl<A: Allocator, C: Continuation<A>> AnonymousSerializer<A, C, ()> {
  pub(crate) fn end(self) -> C {
    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, T> Continuation<A> for AnonymousSerializer<A, C, T> {
  type Seed = C::Seed;

  fn continuation(state: State<A, Self>) -> Self {
    AnonymousSerializer::new(state.identity())
  }
}
