use crate::common::*;

#[must_use]
pub struct Done<A: Allocator> {
  allocator: A,
}

impl<A: Allocator> Done<A> {
  // This inline attribute is required to avoid a compiler ICE as of rustc
  // 1.48.0-nightly (a1947b3f9 2020-09-10). It should be removed once the ICE is
  // fixed.
  #[inline(always)]
  pub fn done(self) -> A::Output {
    self.allocator.finish()
  }
}

impl<A: Allocator> Continuation<A> for Done<A> {
  type State = ();

  fn continuation(allocator: A, _state: Self::State) -> Self {
    Self { allocator }
  }
}
