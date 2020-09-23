use crate::common::*;

pub(crate) trait FromView: X {
  fn from_view(view: &Self::View) -> Self;
}

// TODO: Can I provide this blanket impl?
// impl<'a, T: 'a + X> FromView for T
// where
//   T: From<&'a <T as X>::View>,
// {
//   fn from_view(view: &Self::View) -> Self {
//     todo!()
//   }
// }
