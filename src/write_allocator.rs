use crate::common::*;

pub struct WriteAllocator<W: Write + Seek> {
  writer: W,
  error:  Option<io::Error>,
  offset: usize,
}

impl<W: Write + Seek> WriteAllocator<W> {
  pub fn new(writer: W) -> WriteAllocator<W> {
    WriteAllocator {
      error: None,
      offset: 0,
      writer,
    }
  }
}

impl<W: Write + Seek> Allocator for WriteAllocator<W> {
  type Output = io::Result<()>;

  fn write(&mut self, bytes: &[u8], offset: usize) {
    if self.error.is_some() {
      return;
    }

    if self.offset != offset {
      // TODO: Fix this unwrap
      let seek_from = SeekFrom::Start(offset.try_into().unwrap());

      if let Err(error) = self.writer.seek(seek_from) {
        self.error = Some(error);
        return;
      }
    }

    if let Err(error) = self.writer.write_all(bytes) {
      self.error = Some(error);
      return;
    }

    self.offset += bytes.len();
  }

  fn finish(self) -> Self::Output {
    match self.error {
      None => Ok(()),
      Some(error) => Err(error),
    }
  }
}

#[cfg(test)]
mod tests {
  // TODO: needs tests
}
