use crate::common::*;

pub struct UnitSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for () {
  type Serializer<A: Allocator, C: Continuation<A>> = UnitSerializer<A, C>;
  type View = ();

  fn from_view(view: &Self::View) -> Self {
    *view
  }
}

impl View for () {
  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // Safe because the unit type has no invalid bit patterns.
    Ok(unsafe { suspect.assume_init_ref() })
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
