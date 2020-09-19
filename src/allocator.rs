pub trait Allocator {
  type Output;

  fn write(&mut self, bytes: &[u8], offset: usize);

  fn finish(self) -> Self::Output;
}
