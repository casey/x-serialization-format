use crate::common::*;

pub trait Serializer<A: Allocator, C: Continuation<A>>: Sized {
  // type Input: ?Sized;

  fn new(state: State<A, C>) -> Self;

  // fn serialize<B: Borrow<Self::Input>>(self, native: B) -> C;

  // fn serialize<S: Serialize<Serializer = Self>>(self, serialize: S) -> C {
  //   serialize.serialize(self)
  // }

  fn serialize<N: X<Serializer = Self>>(self, value: &N) -> C {
    value.serialize(self)
  }
}
