use crate::common::*;

pub(crate) trait IntoAllocator<A: Allocator> {
  fn into_allocator(self) -> A;
}
