use crate::common::*;

impl<N: X> X for Vec<N> {
  type Serializer<A: Allocator, C: Continuation<A>> = SliceSerializer<A, C, N>;
  type View = Slice<N::View>;
}

#[repr(C)]
pub struct Slice<V: View> {
  offset: Offset<V>,
  length: Usize,
}

impl<V: View> Slice<V> {
  fn as_slice(&self) -> &[V] {
    let pointer = self.offset.as_ptr();
    unsafe { slice::from_raw_parts(pointer, self.length.to_native()) }
  }
}

impl<'a, V: View> IntoIterator for &'a Slice<V> {
  type IntoIter = slice::Iter<'a, V>;
  type Item = &'a V;

  fn into_iter(self) -> Self::IntoIter {
    self.as_slice().into_iter()
  }
}

impl<V: View> View for Slice<V> {
  type Native = Vec<V::Native>;

  fn to_native(&self) -> Self::Native {
    self.as_slice().iter().map(View::to_native).collect()
  }

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let length: &MaybeUninit<Usize> =
      unsafe { &*((suspect.as_ptr() as *const Offset<V>).add(1) as *const MaybeUninit<Usize>) };

    let length = View::check(length, buffer)?;

    let offset = suspect.cast::<Offset<V>>();

    Offset::check(offset, buffer, length.to_native())?;

    Ok(unsafe { suspect.assume_init_ref() })
  }
}

pub struct SliceSerializer<A: Allocator, C: Continuation<A>, N: X> {
  state:   State<A, C>,
  element: PhantomData<N>,
}

impl<A: Allocator, C: Continuation<A>, N: X> Serializer<A, C> for SliceSerializer<A, C, N> {
  type Native = Vec<N>;

  fn new(state: State<A, C>) -> Self {
    Self {
      element: PhantomData,
      state,
    }
  }

  fn serialize<B: Borrow<Self::Native>>(self, native: B) -> C {
    let native = native.borrow();
    let mut serializer = self.len(native.len());
    for element in native {
      serializer = serializer.element(element);
    }
    serializer.end()
  }
}

impl<A: Allocator, C: Continuation<A>, N: X> SliceSerializer<A, C, N> {
  fn len(mut self, length: usize) -> AllocatedSliceSerializer<A, C, N> {
    let offset = self.state.end();
    self.state.write(&offset.to_u64().to_le_bytes());
    self.state.write(&length.to_u64().to_le_bytes());
    let bytes = mem::size_of::<N::View>() * length;
    self.state.push(bytes);

    AllocatedSliceSerializer {
      serialized: 0,
      state: self.state,
      element: PhantomData,
      length,
    }
  }
}

pub struct AllocatedSliceSerializer<A: Allocator, C: Continuation<A>, N: X> {
  element:    PhantomData<N>,
  length:     usize,
  serialized: usize,
  state:      State<A, C>,
}

impl<A: Allocator, C: Continuation<A>, N: X> AllocatedSliceSerializer<A, C, N> {
  fn element<B: Borrow<N>>(self, native: B) -> Self {
    self.element_serializer().serialize(native)
  }

  fn element_serializer(self) -> <N as X>::Serializer<A, AllocatedSliceSerializer<A, C, N>> {
    if self.length == self.serialized {
      todo!()
    }

    let serialized = self.serialized;
    let length = self.length;

    let state = self.state.transform(|inner| SliceSeed {
      serialized,
      length,
      inner,
    });

    <N as X>::Serializer::new(state)
  }

  fn end(mut self) -> C {
    if self.length != self.serialized {
      todo!()
    }

    self.state.pop();

    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, N: X> Continuation<A> for AllocatedSliceSerializer<A, C, N> {
  type Seed = SliceSeed<A, C>;

  fn continuation(state: State<A, Self>) -> Self {
    let serialized = state.seed().serialized + 1;
    let length = state.seed().length;

    AllocatedSliceSerializer {
      element: PhantomData,
      state: state.transform(|seed| seed.inner),
      length,
      serialized,
    }
  }
}

pub struct SliceSeed<A: Allocator, C: Continuation<A>> {
  serialized: usize,
  length:     usize,
  inner:      C::Seed,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    #[rustfmt::skip]
    ok(vec![0u8, 1, 2, 3], &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      4, 0, 0, 0, 0, 0, 0, 0,
      // elements
      0, 1, 2, 3
    ]);
  }
}