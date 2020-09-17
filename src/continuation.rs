use crate::common::*;

pub trait Continuation<A: Allocator> {
  type State;

  fn continuation(allocator: A, state: Self::State) -> Self;
}
