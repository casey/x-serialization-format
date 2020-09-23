use crate::common::*;

impl<'a, N: X> X for &'a [N] {
  type View = Slice<N::View>;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    serializer.serialize_iterator(self.into_iter())
  }
}

#[repr(C)]
pub struct Slice<V: View> {
  offset: Offset<V>,
  length: Usize,
}

impl<V: View> Slice<V> {
  pub fn as_slice(&self) -> &[V] {
    let pointer = self.offset.as_ptr();
    unsafe { slice::from_raw_parts(pointer, self.length.to_native()) }
  }
}

impl<'a, V: View> IntoIterator for &'a Slice<V> {
  type IntoIter = slice::Iter<'a, V>;
  type Item = &'a V;

  fn into_iter(self) -> Self::IntoIter {
    self.as_slice().iter()
  }
}

impl<V: View> View for Slice<V> {
  type Serializer<A: Allocator, C: Continuation<A>> = SliceSerializer<A, C, V>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let length: &MaybeUninit<Usize> =
      unsafe { &*((suspect.as_ptr() as *const Offset<V>).add(1) as *const MaybeUninit<Usize>) };

    let length = View::check(length, buffer)?;

    let offset = suspect.cast::<Offset<V>>();

    Offset::check(offset, buffer, length.to_native())?;

    Ok(unsafe { suspect.assume_init_ref() })
  }
}

pub struct SliceSerializer<A: Allocator, C: Continuation<A>, V: View> {
  state:   State<A, C>,
  element: PhantomData<V>,
}

impl<A: Allocator, C: Continuation<A>, V: View> Serializer<A, C> for SliceSerializer<A, C, V> {
  fn new(state: State<A, C>) -> Self {
    Self {
      element: PhantomData,
      state,
    }
  }
}

impl<A: Allocator, C: Continuation<A>, V: View> SliceSerializer<A, C, V> {
  pub(crate) fn serialize_iterator<'a, N: 'a + X<View = V>, I: ExactSizeIterator<Item = &'a N>>(
    self,
    iter: I,
  ) -> C {
    let mut serializer = self.len(iter.len());
    for element in iter {
      serializer = serializer.element(element);
    }
    serializer.end()
  }

  fn len(mut self, length: usize) -> AllocatedSliceSerializer<A, C, V> {
    let offset = self.state.end();
    self.state.write(&offset.to_u64().to_le_bytes());
    self.state.write(&length.to_u64().to_le_bytes());
    let bytes = mem::size_of::<V>() * length;
    self.state.push(bytes);

    AllocatedSliceSerializer {
      serialized: 0,
      state: self.state,
      element: PhantomData,
      length,
    }
  }
}

pub struct AllocatedSliceSerializer<A: Allocator, C: Continuation<A>, V: View> {
  element:    PhantomData<V>,
  length:     usize,
  serialized: usize,
  state:      State<A, C>,
}

impl<A: Allocator, C: Continuation<A>, V: View> AllocatedSliceSerializer<A, C, V> {
  fn element<N: X<View = V>>(self, element: &N) -> Self {
    self.element_serializer::<N>().serialize(element)
  }

  fn element_serializer<N: X<View = V>>(
    self,
  ) -> N::Serializer<A, AllocatedSliceSerializer<A, C, V>> {
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

    N::Serializer::new(state)
  }

  fn end(mut self) -> C {
    if self.length != self.serialized {
      todo!()
    }

    self.state.pop();

    self.state.continuation()
  }
}

impl<A: Allocator, C: Continuation<A>, V: View> Continuation<A>
  for AllocatedSliceSerializer<A, C, V>
{
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
    let slice: &[u8] = &[0u8, 1, 2, 3];
    let serialized = slice.serialize_to_vec();

    #[rustfmt::skip]
    assert_eq!(&serialized, &[
      // offset
      16, 0, 0, 0, 0, 0, 0, 0,
      // length
      4, 0, 0, 0, 0, 0, 0, 0,
      // elements
      0, 1, 2, 3
    ]);
  }
}
