use crate::common::*;

#[repr(C)]
#[derive(Debug)]
pub struct Usize {
  inner: U64,
}

impl X for usize {
  type View = Usize;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    // TODO: We should be delegating to U64Serializer, but that causes an ICE
    serializer.state.write(&self.to_u64().to_le_bytes());
    serializer.state.continuation()
  }
}

impl FromView for usize {
  fn from_view(view: &Self::View) -> Self {
    view.into()
  }
}

impl View for Usize {
  type Serializer<A: Allocator, C: Continuation<A>> = UsizeSerializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let struct_pointer: *const Usize = suspect.as_ptr();

    let inner_pointer = struct_pointer as *const U64;

    let inner_suspect_pointer = inner_pointer as *const MaybeUninit<U64>;

    // Safe because:
    // - Alignment is correct: `inner` is a view type, so it has alignment 1.
    // - Pointer is not null: Derived from valid reference `suspect`.
    // - Valid bitpattern: All bitpatterns are valid for MaybeUninit.
    let inner_suspect = unsafe { &*inner_suspect_pointer };

    View::check(inner_suspect, buffer)?;

    // Safe because inner has been checked.
    let reference = unsafe { suspect.assume_init_ref() };

    // Enforce the additional invariant that Usize values may not be larger than can
    // fit in the native `usize` type.

    let value = reference.inner.to_native();

    // TODO: There isn't a test for this
    if value > usize::MAX.to_u64() {
      return Err(Error::Usize { value });
    }

    Ok(reference)
  }
}

impl From<&Usize> for usize {
  fn from(view: &Usize) -> usize {
    u64::from(&view.inner) as usize
  }
}

pub struct UsizeSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for UsizeSerializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    Self { state }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    ok(0usize, &[0, 0, 0, 0, 0, 0, 0, 0]);
    ok(1usize, &[1, 0, 0, 0, 0, 0, 0, 0]);
    ok(usize::MAX, &[
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ]);
  }
}
