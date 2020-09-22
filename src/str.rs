use crate::common::*;

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
  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let slice = suspect.cast::<Slice<u8>>();
    View::check(slice, buffer)?;

    let value = unsafe { suspect.assume_init_ref() };

    value.try_as_str()?;

    Ok(value)
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for StrSerializer<A, C> {
  type Input = str;

  fn new(state: State<A, C>) -> Self {
    Self { state }
  }

  fn serialize<B: Borrow<Self::Input>>(self, native: B) -> C {
    // TODO: This needs to be fixed
    let vec = native.borrow().as_bytes().to_vec();
    SliceSerializer::new(self.state).serialize(vec)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[rustfmt::skip]
  fn basic() {
    ok(String::new(), &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    ok(String::from("\0"), &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      1, 0, 0, 0, 0, 0, 0, 0,
      // contents
      0,
    ]);

    ok(String::from("hello"), &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      5, 0, 0, 0, 0, 0, 0, 0,
      // contents
      104, 101, 108, 108, 111,
    ]);
  }
}
