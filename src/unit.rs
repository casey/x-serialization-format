use crate::common::*;

pub struct UnitSerializer<A: Allocator, C> {
  allocator:    A,
  continuation: PhantomData<C>,
}

impl X for () {
  type Serializer<A: Allocator, C: Continuation<A>> = UnitSerializer<A, C>;
  type View = ();
}

impl View for () {
  type Native = ();

  fn to_native(&self) -> Self::Native {
    ()
  }

  fn check<'value>(_value: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
    Ok(&())
  }
}

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for UnitSerializer<A, C> {
  type Native = ();

  fn new(allocator: A) -> Self {
    UnitSerializer {
      continuation: PhantomData,
      allocator,
    }
  }

  fn serialize<B: Borrow<Self::Native>>(self, _native: B) -> C {
    C::continuation(self.allocator)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn success() {
    ok((), &[]);
  }
}
