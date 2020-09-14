use crate::common::*;

pub use x_derive::X;

pub trait X {
  type View: View<Native = Self>;
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C, Native = Self>;

  fn store<A: Allocator>(allocator: A) -> Self::Serializer<A, Done> {
    Self::Serializer::new(allocator)
  }
}
