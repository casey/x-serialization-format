use crate::common::*;

pub trait Serializer<A: Allocator, C: Continuation<A>> {
  type Native: ?Sized;

  fn new(allocator: A) -> Self;

  fn serialize<B: Borrow<Self::Native>>(self, native: B) -> C;
}
