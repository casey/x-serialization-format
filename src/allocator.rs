pub(crate) trait Allocator {
    fn write(&mut self, bytes: &[u8]);
}
