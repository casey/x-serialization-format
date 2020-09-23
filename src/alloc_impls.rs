use crate::common::*;

impl<N: X> X for Vec<N> {
  type View = Slice<N::View>;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    serializer.serialize_iterator(self.iter())
  }
}

impl<N: X + FromView> FromView for Vec<N> {
  fn from_view(view: &Self::View) -> Self {
    view.as_slice().iter().map(FromView::from_view).collect()
  }
}

impl X for String {
  type View = Str;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    serializer.serialize_str(self)
  }
}

impl FromView for String {
  fn from_view(view: &Self::View) -> Self {
    view.as_str().into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vec() {
    #[rustfmt::skip]
    ok(vec![0u8, 1, 2, 3], &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      4, 0, 0, 0, 0, 0, 0, 0,
      // elements
      0, 1, 2, 3
    ]);
  }

  #[test]
  #[rustfmt::skip]
  fn string() {
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
