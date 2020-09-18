#![allow(unused)]

pub(crate) struct Stack<S> {
  offsets: S,
}

impl<S> Stack<S> {
  fn push(self, offset: usize) -> Stack<(usize, S)> {
    Stack {
      offsets: (offset, self.offsets),
    }
  }
}

impl<T> Stack<(usize, T)> {
  fn pop(self) -> Stack<T> {
    Stack {
      offsets: self.offsets.1,
    }
  }

  fn top(&self) -> usize {
    self.offsets.0
  }

  fn increment(&mut self, increment: usize) {
    self.offsets.0 += increment
  }
}
