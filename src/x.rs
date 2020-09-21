use crate::common::*;

pub use x_derive::X;

pub trait X {
  type View: View<Native = Self>;
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C, Native = Self>;

  fn store<A: Allocator>(allocator: A) -> Self::Serializer<A, Done<A>> {
    let mut state = State::new(allocator, ());

    // Allocate space for the root object:
    state.push(mem::size_of::<Self::View>());

    // TODO: who performs the corresponding pop? conceptually, it would be nice if
    // the stack always ended empty.

    // Return the serializer:
    Self::Serializer::new(state)
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

  fn view(buffer: &[u8]) -> Result<&Self::View> {
    Self::View::load(buffer)
  }
}
