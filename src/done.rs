use crate::common::*;

#[must_use]
pub struct Done;

impl Done {
  // This inline attribute is required to avoid a compiler ICE as of rustc
  // 1.48.0-nightly (a1947b3f9 2020-09-10). It should be removed once the ICE is
  // fixed.
  #[inline(always)]
  pub fn done(self) {}
}

impl<A: Allocator> Continuation<A> for Done {
  fn continuation(allocator: A) -> Self {
    Done
  }
}
