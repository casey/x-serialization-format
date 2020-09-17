use crate::common::*;

pub trait Serializer<A: Allocator, C: Continuation<A>> {
  type Native: ?Sized;

  fn new(state: State<A, C>) -> Self;

  fn serialize<B: Borrow<Self::Native>>(self, native: B) -> C;
}
