pub trait Allocator {
  type Output;

  fn write(&mut self, bytes: &[u8]);

  fn finish(self) -> Self::Output;
}
