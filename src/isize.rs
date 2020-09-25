use crate::common::*;

impl X for isize {
  type View = Isize;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: <Self::View as View>::Serializer<A, C>,
  ) -> C {
    // TODO: We should be delegating to I64Serializer, but that causes an ICE
    serializer.state.write(&self.to_i64().to_le_bytes());
    serializer.state.continuation()
  }
}

impl FromView for isize {
  fn from_view(view: &Self::View) -> Self {
    view.into()
  }
}

#[repr(C)]
pub struct Isize {
  inner: I64,
}

impl View for Isize {
  type Serializer<A: Allocator, C: Continuation<A>> = IsizeSerializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let struct_pointer: *const Isize = suspect.as_ptr();

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

    // Enforce the additional invariant that Iszie values may not be larger than can
    // fit in the native `usize` type.

    let value = reference.inner.to_native();

    // TODO: There isn't a test for this
    if value < isize::MIN.to_i64() || value > isize::MAX.to_i64() {
      return Err(Error::Isize { value });
    }

    Ok(reference)
  }
}

impl From<&Isize> for isize {
  fn from(view: &Isize) -> isize {
    i64::from(&view.inner) as isize
  }
}

pub struct IsizeSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for IsizeSerializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    Self { state }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    ok(0isize, &[0, 0, 0, 0, 0, 0, 0, 0]);
    ok(1isize, &[1, 0, 0, 0, 0, 0, 0, 0]);
    ok(-1isize, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    ok(isize::MAX, &[
      0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F,
    ]);
    ok(isize::MIN, &[0, 0, 0, 0, 0, 0, 0, 0x80]);
  }
}
