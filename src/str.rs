use crate::common::*;

impl<'a> X for &'a str {
  type View = Str;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    serializer.serialize_str(self)
  }
}

pub struct Str {
  slice: Slice<u8>,
}

pub struct StrSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl Str {
  pub fn as_str(&self) -> &str {
    self.try_as_str().unwrap()
  }

  fn try_as_str(&self) -> Result<&str> {
    Ok(str::from_utf8(self.slice.as_slice())?)
  }
}

impl<'a> From<&'a Str> for &'a str {
  fn from(view: &'a Str) -> Self {
    view.try_as_str().unwrap()
  }
}

impl View for Str {
  type Serializer<A: Allocator, C: Continuation<A>> = StrSerializer<A, C>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let slice = suspect.cast::<Slice<u8>>();
    View::check(slice, buffer)?;

    let value = unsafe { suspect.assume_init_ref() };

    value.try_as_str()?;

    Ok(value)
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for StrSerializer<A, C> {
  fn new(state: State<A, C>) -> Self {
    Self { state }
  }
}

impl<A: Allocator, C: Continuation<A>> StrSerializer<A, C> {
  pub(crate) fn serialize_str(self, string: &str) -> C {
    // TODO: This should just call .serialize, but there's an ICE
    SliceSerializer::<A, C, u8>::new(self.state).serialize_iterator(string.as_bytes().into_iter())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[rustfmt::skip]
  fn basic() {
    ok_serialize("", &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    ok_serialize("\0", &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      1, 0, 0, 0, 0, 0, 0, 0,
      // contents
      0,
    ]);

    ok_serialize("hello", &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      5, 0, 0, 0, 0, 0, 0, 0,
      // contents
      104, 101, 108, 108, 111,
    ]);
  }
}
