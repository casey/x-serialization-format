use crate::common::*;

pub trait Continuation<A: Allocator> {
  type Seed;

  fn continuation(allocator: A, seed: Self::Seed) -> Self;
}
