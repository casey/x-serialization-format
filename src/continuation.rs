use crate::common::*;

pub trait Continuation<A: Allocator> {
  fn continuation(allocator: A) -> Self;
}
