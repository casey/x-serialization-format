use crate::common::*;

#[must_use]
pub struct Done;

impl Done {
  pub fn done(self) {}
}

impl<A: Allocator> Continuation<A> for Done {
  fn continuation(allocator: A) -> Self {
    Done
  }
}
