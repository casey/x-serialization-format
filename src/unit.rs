use crate::common::*;

pub struct UnitSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for () {
  type View = ();

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    serializer.state.continuation()
  }
}

impl FromView for () {
  fn from_view(view: &Self::View) -> Self {
    *view
  }
}

impl View for () {
  type Serializer<A: Allocator, C: Continuation<A>> = UnitSerializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    // Safe because the unit type has no invalid bit patterns.
    Ok(unsafe { suspect.assume_init_ref() })
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for UnitSerializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    UnitSerializer { state }
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
