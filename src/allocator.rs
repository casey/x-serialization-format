pub trait Allocator {
  fn write(&mut self, bytes: &[u8]);
}
