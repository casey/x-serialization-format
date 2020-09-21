use crate::common::*;

pub(crate) struct PaddingSerializer<A: Allocator, C: Continuation<A>, S: View, U: View> {
  state: State<A, C>,
  s:     PhantomData<S>,
  u:     PhantomData<U>,
}

impl<A: Allocator, C: Continuation<A>, S: View, U: View> PaddingSerializer<A, C, S, U> {
  pub(crate) fn serialize_padding(mut self) -> C {
    let zeroed: MaybeUninit<U> = MaybeUninit::zeroed();
    let padding = mem::size_of::<U>().saturating_sub(mem::size_of::<S>());
    let pointer: *const MaybeUninit<U> = &zeroed;
    let serialized = mem::size_of::<U>() - padding;
    let bytes = unsafe { (pointer as *const u8).add(serialized) };
    let slice = unsafe { slice::from_raw_parts(bytes, padding) };
    self.state.write(slice);
    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, S: View, U: View> Continuation<A>
  for PaddingSerializer<A, C, S, U>
{
  type Seed = C::Seed;

  fn continuation(state: State<A, Self>) -> Self {
    Self {
      s:     PhantomData,
      u:     PhantomData,
      state: state.identity(),
    }
  }
}
