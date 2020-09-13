use crate::common::*;

#[must_use]
pub(crate) struct Done;

impl Done {
  pub(crate) fn done(self) {}
}

impl<A: Allocator> Continuation<A> for Done {
  fn continuation(allocator: A) -> Self {
    Done
  }
}
