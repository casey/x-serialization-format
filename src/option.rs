use crate::common::*;

// TODO:
// should serializer get .none and .some methods?

impl<N: X> X for core::option::Option<N> {
  type View = self::Option<N::View>;

  fn serialize<A: Allocator, C: Continuation<A>>(
    &self,
    mut serializer: Self::Serializer<A, C>,
  ) -> C {
    match self {
      None => {
        assert_eq!(NONE_DISCRIMINANT, 0);
        // We take advantage of the fact that None's discriminant is zero, and just emit
        // a fully zeroed value:
        let mut value = self::Option::<N::View>::None;
        unsafe { ptr::write_bytes(&mut value, 0, 1) };
        let pointer: *const self::Option<N::View> = &value;
        let pointer = pointer as *const u8;
        let bytes: &[u8] =
          unsafe { slice::from_raw_parts(pointer, mem::size_of::<self::Option<N::View>>()) };

        serializer.state.write(bytes);
        serializer.state.continuation()
      },
      Some(t) => {
        serializer.state.write(&[SOME_DISCRIMINANT]);
        N::Serializer::new(serializer.state).serialize(t)
      },
    }
  }
}

const NONE_DISCRIMINANT: u8 = 0;
const SOME_DISCRIMINANT: u8 = 1;

#[repr(u8)]
#[derive(Debug)]
pub enum Option<V: View> {
  None = NONE_DISCRIMINANT,
  Some(V) = SOME_DISCRIMINANT,
}

impl<V: View> View for self::Option<V> {
  type Serializer<A: Allocator, C: Continuation<A>> = OptionSerializer<A, C, V>;

  fn check<'value>(suspect: &'value MaybeUninit<Self>, buffer: &[u8]) -> Result<&'value Self> {
    let pointer = suspect.as_ptr() as *const u8;

    let discriminant = unsafe { *pointer };

    match discriminant {
      NONE_DISCRIMINANT => Ok(unsafe { suspect.assume_init_ref() }),
      SOME_DISCRIMINANT => {
        let payload = unsafe { pointer.add(1) } as *const MaybeUninit<V>;
        View::check(unsafe { &*payload }, buffer)?;
        Ok(unsafe { suspect.assume_init_ref() })
      },
      value => Err(Error::Discriminant {
        maximum: SOME_DISCRIMINANT,
        ty: "Option",
        value,
      }),
    }
  }
}

pub struct OptionSerializer<A: Allocator, C: Continuation<A>, V: View> {
  state: State<A, C>,
  data:  PhantomData<V>,
}

impl<A: Allocator, C: Continuation<A>, V: View> Serializer<A, C> for OptionSerializer<A, C, V> {
  fn new(state: State<A, C>) -> Self {
    Self {
      data: PhantomData,
      state,
    }
  }
}

impl<T: FromView> FromView for core::option::Option<T> {
  fn from_view(view: &Self::View) -> Self {
    match view {
      self::Option::None => None,
      self::Option::Some(t) => Some(FromView::from_view(t)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic() {
    ok(core::option::Option::<char>::None, &[0, 0, 0, 0]);
    ok(core::option::Option::<char>::Some('a'), &[1, 97, 0, 0]);
  }

  #[test]
  fn invalid_discriminant() {
    assert_eq!(
      core::option::Option::<u8>::view(&[2, 0]).unwrap_err(),
      Error::Discriminant {
        value:   2,
        maximum: SOME_DISCRIMINANT,
        ty:      "Option",
      }
    );
  }

  #[test]
  fn invalid_payload() {
    assert_eq!(
      core::option::Option::<char>::view(&[1, 0xFF, 0xFF, 0xFF]).unwrap_err(),
      Error::Char { value: 0xFFFFFF }
    );
  }
}
