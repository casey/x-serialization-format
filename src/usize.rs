use crate::common::*;

impl X for usize {
  type Serializer<A: Allocator, C: Continuation<A>> = UsizeSerializer<A, C>;
  type View = Usize;

  fn from_view(view: &Self::View) -> Self {
    u64::from_view(&view.inner) as usize
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct Usize {
  inner: U64,
}

impl View for Usize {
  type Native = usize;

  fn to_native(&self) -> Self::Native {
    self.inner.to_native() as usize
  }

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

pub struct UsizeSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for UsizeSerializer<A, C> {
  type Native = usize;

  fn new(state: State<A, C>) -> Self {
    Self { state }
  }

  fn serialize<B: Borrow<Self::Native>>(self, native: B) -> C {
    let native = native.borrow();
    U64Serializer::new(self.state).serialize(native.to_u64())
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
