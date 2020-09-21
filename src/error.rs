use crate::common::*;

// Document that this is load errors only.
// TODO: lint that all variants are documented?
#[derive(Debug, PartialEq)]
pub enum Error {
  OffsetBounds {
    buffer: Range<*const u8>,
    offset: Range<*const u8>,
  },
  // offset is zero where a zero offset is not valid
  OffsetNull,
  // offset value is invalid, > 0 and < 8
  OffsetValue {
    value: usize,
  },
  // offset pointer + element size wraps memory space
  OffsetWrap {
    start: *const u8,
    end:   *const u8,
  },
  // offset points before the beginning of the buffer or past the end of the buffer
  OffsetElementBounds {
    buffer:   Range<*const u8>,
    elements: Range<*const u8>,
  },
  Usize {
    value: u64,
  },
  Isize {
    value: i64,
  },
  // TODO: Come up with a better name
  BufferTooSmall,
  Bool {
    value: u8,
  },
  Char {
    value: u32,
  },
  String {
    error: Utf8Error,
  },
  Discriminant {
    value:   u8,
    maximum: u8,
    ty:      &'static str,
  },
}

impl From<Utf8Error> for Error {
  fn from(error: Utf8Error) -> Self {
    Error::String { error }
  }
}
