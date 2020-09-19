use crate::common::*;

pub trait Continuation<A: Allocator>: Sized {
  type Seed;

  fn continuation(state: State<A, Self>) -> Self;
}
