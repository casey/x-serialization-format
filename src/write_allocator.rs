use crate::common::*;

pub struct WriteAllocator<W: Write + Seek> {
  writer: W,
  error:  Option<io::Error>,
}

impl<W: Write + Seek> WriteAllocator<W> {
  pub fn new(writer: W) -> WriteAllocator<W> {
    WriteAllocator {
      error: None,
      writer,
    }
  }
}

impl<W: Write + Seek> Allocator for WriteAllocator<W> {
  type Output = io::Result<()>;

  fn write(&mut self, bytes: &[u8]) {
    if self.error.is_some() {
      return;
    }

    if let Err(error) = self.writer.write_all(bytes) {
      self.error = Some(error);
    }
  }

  fn finish(self) -> Self::Output {
    match self.error {
      None => Ok(()),
      Some(error) => Err(error),
    }
  }
}
