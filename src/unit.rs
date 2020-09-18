use crate::common::*;

pub struct UnitSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for () {
  type Serializer<A: Allocator, C: Continuation<A>> = UnitSerializer<A, C>;
  type View = ();
}

impl View for () {
  type Native = ();

  fn to_native(&self) -> Self::Native {
    ()
  }

  fn check<'value>(_suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    Ok(&())
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for UnitSerializer<A, C> {
  type Native = ();

  fn new(state: State<A, C>) -> Self {
    UnitSerializer { state }
  }

  fn serialize<B: Borrow<Self::Native>>(self, _native: B) -> C {
    self.state.continuation()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    ok((), &[]);
  }
}
