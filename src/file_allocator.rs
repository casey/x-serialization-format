use crate::common::*;

// TODO: document that anything that implements write and seek can be used
pub struct FileAllocator<F: Write + Seek> {
  file:     F,
  error:    Option<io::Error>,
  position: usize,
  end:      usize,
}

impl<F: Write + Seek> FileAllocator<F> {
  pub fn new(file: F) -> FileAllocator<F> {
    Self {
      error: None,
      position: 0,
      end: 0,
      file,
    }
  }
}

impl<F: Write + Seek> Allocator for FileAllocator<F> {
  type Output = io::Result<()>;

  fn write(&mut self, bytes: &[u8], offset: usize) {
    if self.error.is_some() {
      return;
    }

    if self.position != offset {
      // TODO: Fix this unwrap
      let seek_from = SeekFrom::Start(offset.try_into().unwrap());

      if let Err(error) = self.file.seek(seek_from) {
        self.error = Some(error);
        return;
      }
    }

    if let Err(error) = self.file.write_all(bytes) {
      self.error = Some(error);
      return;
    }

    self.position = offset + bytes.len();
    self.end = self.end.max(self.position);
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
