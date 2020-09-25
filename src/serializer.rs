use crate::common::*;

pub trait Serializer<A: Allocator, C: Continuation<A>>: Sized {
  fn new(state: State<A, C>) -> Self;

  fn serialize<N, V>(self, value: &N) -> C
  where
    N: X<View = V>,
    V: View<Serializer = Self>,
  {
    value.serialize(self)
  }
}
