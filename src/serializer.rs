use crate::common::*;

pub trait Serializer<A: Allocator, C: Continuation<A>>: Sized {
  fn new(state: State<A, C>) -> Self;

  fn serialize<N: X<Serializer = Self>>(self, value: &N) -> C {
    value.serialize(self)
  }
}
