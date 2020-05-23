use crate::common::*;

pub(crate) struct Structure {
  ident: Ident,
  input: DataStruct,
}

impl Structure {
  pub(crate) fn new(ident: Ident, input: DataStruct) -> Structure {
    Structure { ident, input }
  }

  fn field_types(&self) -> Vec<Type> {
    self
      .input
      .fields
      .iter()
      .map(|field| field.ty.clone())
      .collect()
  }
}

impl Tokens for Structure {
  fn tokens(self) -> TokenStream {
    let ident = self.ident.clone();

    let field_types_vec = self.field_types();

    let field_types = &field_types_vec;

    quote!(
      impl<'a> ::data::Data<'a> for #ident {
        type Array = [u8; 0 #( + <#field_types as ::data::Data>::FIXED_SIZE )*];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 0 #( + <#field_types as ::data::Data>::FIXED_SIZE )* ;

        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    )
  }
}

#[cfg(test)]
mod tests {
  macro_rules! assert_derive_data_expansion_eq {
    {
      $item:item,
      $($expansion:tt)*
    } => {
      {
        let have = crate::derive_data_inner(quote::quote!($item)).unwrap().to_string();
        let want = quote::quote!($($expansion)*).to_string();
        pretty_assertions::assert_eq!(have, want);
      }
    }
  }

  #[test]
  fn unit_derive() {
    assert_derive_data_expansion_eq!(
      struct Foo;,
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 0;

        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }

  #[test]
  fn named_empty() {
    assert_derive_data_expansion_eq!(
      struct Foo {},
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 0;

        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }

  #[test]
  fn unnamed_empty() {
    assert_derive_data_expansion_eq!(
      struct Foo();,
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 0;

        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }

  #[test]
  fn named_single() {
    #[rustfmt::skip]
    assert_derive_data_expansion_eq!(
      struct Foo {
        foo: u64,
      },
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0 + <u64 as ::data::Data>::FIXED_SIZE];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 0 + <u64 as ::data::Data>::FIXED_SIZE;

        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }

  #[test]
  fn unnamed_single() {
    #[rustfmt::skip]
    assert_derive_data_expansion_eq!(
      struct Foo(u64);,
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0 + <u64 as ::data::Data>::FIXED_SIZE];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 0 + <u64 as ::data::Data>::FIXED_SIZE;
        
        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }

  #[test]
  fn named_multiple() {
    #[rustfmt::skip]
    assert_derive_data_expansion_eq!(
      struct Foo {
        foo: u64,
        bar: u8,
      },
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0 + <u64 as ::data::Data>::FIXED_SIZE + <u8 as ::data::Data>::FIXED_SIZE];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 
          0 + <u64 as ::data::Data>::FIXED_SIZE +  <u8 as ::data::Data>::FIXED_SIZE;
        
        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }

  #[test]
  fn unnamed_multiple() {
    #[rustfmt::skip]
    assert_derive_data_expansion_eq!(
      #[data::structure]
      struct Foo(u64, u8);,
      impl<'a> ::data::Data<'a> for Foo {
        type Array = [u8; 0 + <u64 as ::data::Data>::FIXED_SIZE + <u8 as ::data::Data>::FIXED_SIZE];
        type ArrayMut = &'a mut Self::Array;
        type ArrayRef = &'a Self::Array;

        const FIXED_SIZE: usize = 
          0 + <u64 as ::data::Data>::FIXED_SIZE +  <u8 as ::data::Data>::FIXED_SIZE;
        
        fn load(buffer: &'a [u8]) -> ::data::Result<Self> {
          todo!();
        }

        fn store(&self, buffer: &mut [u8]) -> ::data::Result<()> {
          todo!()
        }
      }
    );
  }
}
