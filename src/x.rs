use crate::common::*;

pub use x_derive::X;

// TODO: Implement from &View for all applicable types
// TODO: Can I get both ref and value working for serialize methods?
pub trait X: Sized {
  type View: View;

  // TODO: Can I get rid of the as X and just use as View?
  fn store<A: Allocator>(allocator: A) -> <Self::View as View>::Serializer<A, Done<A>> {
    let mut state = State::new(allocator, ());

    // Allocate space for the root object:
    state.push(mem::size_of::<Self::View>());

    // Return the serializer:
    <Self::View as View>::Serializer::new(state)
  }

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C;

  fn store_to_slice(
    slice: &mut [u8],
  ) -> <Self::View as View>::Serializer<SliceAllocator, Done<SliceAllocator>> {
    Self::store(SliceAllocator::new(slice))
  }

  #[cfg(feature = "alloc")]
  fn store_to_vec() -> <Self::View as View>::Serializer<VecAllocator, Done<VecAllocator>> {
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

impl<T: X> X for &T {
  type View = <T as X>::View;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    T::serialize(*self, serializer)
  }
}
