use crate::common::*;

pub struct StringView {
  slice: Slice<u8>,
}

pub struct StringSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for String {
  type Serializer<A: Allocator, C: Continuation<A>> = StringSerializer<A, C>;
  type View = StringView;
}

impl View for StringView {
  type Native = String;

  fn to_native(&self) -> Self::Native {
    todo!()
  }

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    todo!()
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for StringSerializer<A, C> {
  type Native = String;

  fn new(state: State<A, C>) -> Self {
    Self { state }
  }

  fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
    panic!()
  }
}
