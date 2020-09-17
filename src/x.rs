use crate::common::*;

pub use x_derive::X;

pub trait X {
  type View: View<Native = Self>;
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C, Native = Self>;

  fn store<A: Allocator>(allocator: A) -> Self::Serializer<A, Done<A>> {
    Self::Serializer::new(State::new(allocator, ()))
  }

  fn store_to_slice(slice: &mut [u8]) -> Self::Serializer<SliceAllocator, Done<SliceAllocator>> {
    Self::store(SliceAllocator::new(slice))
  }

  #[cfg(feature = "alloc")]
  fn store_to_vec() -> Self::Serializer<VecAllocator, Done<VecAllocator>> {
    Self::store(VecAllocator::new())
  }

  #[cfg(feature = "alloc")]
  fn serialize_to_vec(&self) -> Vec<u8> {
    Self::store(VecAllocator::new()).serialize(self).done()
  }
}
