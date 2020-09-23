use crate::common::*;

pub trait Serialize {
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C>;

  fn serialize<A: Allocator, C: Continuation<A>>(&self, serializer: Self::Serializer<A, C>) -> C;
}
