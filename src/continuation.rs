use crate::common::*;

pub(crate) trait Continuation<A: Allocator> {
  fn continuation(allocator: A) -> Self;
}
