use super::*;

pub trait ToNative<N> {
  fn to_native(&self) -> N;
}

impl<N, V> ToNative<N> for V
where
  N: X<View = V>,
{
  fn to_native(&self) -> N {
    N::from_view(self)
  }
}
