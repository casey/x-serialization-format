use crate::common::*;

pub(crate) use x_derive::X;

pub(crate) trait X {
  type View;
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C, Native = Self>;

  fn store<A: Allocator>(allocator: A) -> Self::Serializer<A, Done> {
    Self::Serializer::new(allocator)
  }
}
