use crate::common::*;

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Error {
  // view type alignment not 1, this error is indicative of a bug,
  // not sure if this error should exist.
  // `alignment` is the alignment of the view type
  Alignment {
    alignment: usize,
  },
  // tried to load a value from a buffer that was too small
  // `over` is the amount missing from the buffer
  Bounds {
    over: usize,
  },
  // tried to store a value in a buffer that was too small,
  // `buffer_size` is the size of the buffer, `total_size` is the
  // size of the value being stored.
  Space {
    buffer_size: usize,
    total_size: usize,
  },
  // attemped to allocate more space then remaining in the allocator
  // `have` is how much space is remaining
  // `want` is how much space was requiested
  Allocate {
    have: usize,
    want: usize,
  },
  // tried to load a `Size` value that could not fit into the platforms
  // `usize`. This depends on teh platform's bit width. messages that would
  // successfully load on a 64 bit platform, might not load on a 32 bit platform
  // `value` is the value that was loaded
  Size {
    value: u64,
  },
  // value was not in a buffer when it could be
  // this currently only happens if an Offset::check is called with a buffer
  // that does not contain the offset.
  // `buffer` is the start and end of the buffer
  // `value` is the start and end of the value
  ValueNotInBuffer {
    buffer: Range<*const u8>,
    value: Range<*const u8>,
  },
  // an offset overflows usize
  // offsets are relative to themselevs
  // this means pointer to offset + value to offset overflowed size
  OffsetOverflow {
    pointer: *const u8,
    size: usize,
  },
  // an offset points past the end of a buffer
  OffsetBounds {
    end: *const u8,
    offset: usize,
  },
  // slice length overflows `usize`, i.e. element size * element count > usize max
  SliceLenOverflow {
    len: usize,
    element_size: usize,
  },
  // slice end, which is pointer to slice start + byte count, overflows usize
  SliceEndOverflow {
    start: *const u8,
    bytes: usize,
  },
  // slice end points past end of buffer
  SliceBounds {
    end: *const u8,
    buffer_end: *const u8,
  },
  // string not valid utf8
  StringDecode {
    source: Utf8Error,
  },
  // an internal error, that indicates a bug in the library
  Internal {
    message: &'static str,
  },
}

impl Error {
  /// Construct an internal error with the given message
  pub(crate) fn internal(message: &'static str) -> Error {
    Error::Internal { message }
  }
}
