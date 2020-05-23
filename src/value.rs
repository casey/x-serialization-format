pub trait Value<'a> {
  type Value;

  fn value(&'a self) -> Self::Value;
}
