use crate::common::*;

pub use x_derive::X;

// TODO: I'm using borrow here just so I can pass T and &T to serializers. Is
// there a better way?
pub trait X: Sized {
  type View: View;

  // This just avoids a lot of nasty typing
  type Serializer<A: Allocator, C: Continuation<A>>: Serializer<A, C> =
    <<Self as X>::View as View>::Serializer<A, C>;

  // TODO: Remove this
  fn from_view(view: &Self::View) -> Self;

  fn store<A: Allocator>(allocator: A) -> Self::Serializer<A, Done<A>> {
    let mut state = State::new(allocator, ());

    // Allocate space for the root object:
    state.push(mem::size_of::<Self::View>());

    // Return the serializer:
    Self::Serializer::new(state)
  }

  fn serialize<A: Allocator, C: Continuation<A>>(&self, serializer: Self::Serializer<A, C>) -> C;

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
