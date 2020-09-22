use crate::common::*;

pub struct String {
  slice: Slice<u8>,
}

pub struct StringSerializer<A: Allocator, C: Continuation<A>> {
  state: State<A, C>,
}

impl X for alloc::string::String {
  type Serializer<A: Allocator, C: Continuation<A>> = StringSerializer<A, C>;
  type View = self::String;

  fn from_view(view: &Self::View) -> Self {
    view.as_str().into()
  }
}

impl self::String {
  pub fn as_str(&self) -> &str {
    self.try_as_str().unwrap()
  }

  fn try_as_str(&self) -> Result<&str> {
    Ok(str::from_utf8(self.slice.as_slice())?)
  }
}

impl View for self::String {
  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let slice = suspect.cast::<Slice<u8>>();
    View::check(slice, buffer)?;

    let value = unsafe { suspect.assume_init_ref() };

    value.try_as_str()?;

    Ok(value)
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for StringSerializer<A, C> {
  type Native = alloc::string::String;

  fn new(state: State<A, C>) -> Self {
    Self { state }
  }

  fn serialize<B: Borrow<Self::Native>>(self, native: B) -> C {
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
    ok(alloc::string::String::new(), &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    ok(alloc::string::String::from("\0"), &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      1, 0, 0, 0, 0, 0, 0, 0,
      // contents
      0,
    ]);

    ok(alloc::string::String::from("hello"), &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      5, 0, 0, 0, 0, 0, 0, 0,
      // contents
      104, 101, 108, 108, 111,
    ]);
  }
}
