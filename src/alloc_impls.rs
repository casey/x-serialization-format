use crate::common::*;

impl<N: X> X for Vec<N> {
  type Borrowed = [N];
  type Serializer<A: Allocator, C: Continuation<A>> = SliceSerializer<A, C, N>;
  type View = Slice<N::View>;

  fn from_view(view: &Self::View) -> Self {
    view.as_slice().iter().map(X::from_view).collect()
  }
}

impl<'a, T, V: View> From<&'a Slice<V>> for Vec<T>
where
  T: From<&'a V>,
{
  fn from(view: &'a Slice<V>) -> Self {
    view.as_slice().iter().map(Into::into).collect()
  }
}

impl X for String {
  type Borrowed = str;
  type Serializer<A: Allocator, C: Continuation<A>> = StrSerializer<A, C>;
  type View = Str;

  fn from_view(view: &Self::View) -> Self {
    view.as_str().into()
  }
}

impl<'a> From<&'a Str> for String {
  fn from(view: &'a Str) -> Self {
    let native: &'a str = view.into();
    native.into()
  }
}
