use crate::common::*;

macro_rules! integer {
  {
    native:     $native:ident,
    view:       $view:ident,
    serializer: $serializer:ident
  } => {
    #[repr(C)]
    #[derive(Debug)]
    pub struct $view {
      le_bytes: [u8; mem::size_of::<$native>()],
    }

    pub struct $serializer<A: Allocator, C> {
      allocator: A,
      continuation: PhantomData<C>,
    }

    impl X for $native {
      type Serializer<A: Allocator, C: Continuation<A>> = $serializer<A, C>;
      type View = $view;
    }

    impl View for $view {
      type Native = $native;

      fn to_native(&self) -> Self::Native {
        $native::from_le_bytes(self.le_bytes)
      }

      fn check<'value>(value: &'value MaybeUninit<Self>, _buffer: &[u8]) -> Result<&'value Self> {
        // This is safe because all bitpatterns of the correct size are valid values of type Self.
        Ok(unsafe { value.assume_init_ref() })
      }
    }

    impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for $serializer<A, C> {
      type Native = $native;

      fn new(allocator: A) -> Self {
        $serializer {
          continuation: PhantomData,
          allocator,
        }
      }

      fn serialize<B: Borrow<Self::Native>>(mut self, native: B) -> C {
        self.allocator.write(&native.borrow().to_le_bytes());
        C::continuation(self.allocator)
      }
    }
  }
}

integer! { native: u16,  view: U16,  serializer: U16Serializer  }
integer! { native: u32,  view: U32,  serializer: U32Serializer  }
integer! { native: u64,  view: U64,  serializer: U64Serializer  }
integer! { native: u128, view: U128, serializer: U128Serializer }
integer! { native: i16,  view: I16,  serializer: I16Serializer  }
integer! { native: i32,  view: I32,  serializer: I32Serializer  }
integer! { native: i64,  view: I64,  serializer: I64Serializer  }
integer! { native: i128, view: I128, serializer: I128Serializer }

#[cfg(test)]
mod tests {
  use super::*;

  // TODO:
  // - check that deserialization fails when buffer is wrong size
  // - and returns wrong error

  #[test]
  #[rustfmt::skip]
  fn success() {
    ok( 0u16,     &[0x00, 0x00]);
    ok( 1u16,     &[0x01, 0x00]);
    ok( u16::MAX, &[0xFF, 0xFF]);
    ok( 0i16,     &[0x00, 0x00]);
    ok( 1i16,     &[0x01, 0x00]);
    ok(-1i16,     &[0xFF, 0xFF]);
    ok( i16::MAX, &[0xFF, 0x7F]);
    ok( i16::MIN, &[0x00, 0x80]);

    ok( 0u32,     &[0x00, 0x00, 0x00, 0x00]);
    ok( 1u32,     &[0x01, 0x00, 0x00, 0x00]);
    ok( u32::MAX, &[0xFF, 0xFF, 0xFF, 0xFF]);
    ok( 0i32,     &[0x00, 0x00, 0x00, 0x00]);
    ok( 1i32,     &[0x01, 0x00, 0x00, 0x00]);
    ok(-1i32,     &[0xFF, 0xFF, 0xFF, 0xFF]);
    ok( i32::MAX, &[0xFF, 0xFF, 0xFF, 0x7F]);
    ok( i32::MIN, &[0x00, 0x00, 0x00, 0x80]);
    
    ok( 0u64,     &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok( 1u64,     &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok( u64::MAX, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    ok( 0i64,     &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok( 1i64,     &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok(-1i64,     &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    ok( i64::MAX, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
    ok( i64::MIN, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]);
    
    ok( 0u128,     &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok( 1u128,     &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok( u128::MAX, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    ok( 0i128,     &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok( 1i128,     &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    ok(-1i128,     &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    ok( i128::MAX, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]);
    ok( i128::MIN, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]);
  }
}
