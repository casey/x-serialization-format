use crate::common::*;

pub(crate) trait X {
  type View;
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C, Native = Self>;
}
