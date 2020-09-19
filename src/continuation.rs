use crate::common::*;

pub trait Continuation<A: Allocator>: Sized {
  type Seed;

  fn continuation(allocator: A, seed: Self::Seed) -> Self;

  fn continuation_from_state(state: State<A, Self>) -> Self {
    panic!()
  }
}
