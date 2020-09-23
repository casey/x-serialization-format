use crate::common::*;

pub trait FromView: X {
  fn from_view(view: &Self::View) -> Self;
}
