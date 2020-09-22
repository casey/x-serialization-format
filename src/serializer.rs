use crate::common::*;

pub trait Serializer<A: Allocator, C: Continuation<A>> {
  type Input: ?Sized;

  fn new(state: State<A, C>) -> Self;

  fn serialize<B: Borrow<Self::Input>>(self, native: B) -> C;
}
