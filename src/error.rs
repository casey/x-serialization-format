#[derive(Debug, PartialEq)]
pub enum Error {
  // TODO: Come up with a better name
  BufferTooSmall,
  Bool { value: u8 },
}
