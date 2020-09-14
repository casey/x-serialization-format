pub trait View {
  type Native;

  fn to_native(&self) -> Self::Native;
}
