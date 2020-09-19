use crate::common::*;

pub struct WriteAllocator<W: Write + Seek> {
  writer: W,
  error:  Option<io::Error>,
  end:    usize,
}

impl<W: Write + Seek> WriteAllocator<W> {
  pub fn new(writer: W) -> WriteAllocator<W> {
    WriteAllocator {
      error: None,
      end: 0,
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

    if self.end != offset {
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

    self.end = offset + bytes.len();
  }

  fn finish(self, end: usize) -> Self::Output {
    match self.error {
      None => {
        assert_eq!(self.end, end);
        Ok(())
      },
      Some(error) => Err(error),
    }
  }
}

#[cfg(test)]
mod tests {
  // TODO: needs tests
}
