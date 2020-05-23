use crate::common::*;

pub(crate) struct Structure {
  ident: Ident,
  input: DataStruct,
  field_accessors: Vec<TokenTree>,
  field_types: Vec<Type>,
  lifetimes: Vec<Lifetime>,
}

impl Structure {
  pub(crate) fn new(ident: Ident, lifetimes: Vec<Lifetime>, input: DataStruct) -> Structure {
    let field_types = input.fields.iter().map(|field| field.ty.clone()).collect();

    let field_accessors = match &input.fields {
      Fields::Named(fields) => fields
        .named
        .iter()
        .flat_map(|field| field.ident.to_token_stream())
        .collect(),
      Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .flat_map(|(index, _)| syn::Index::from(index).into_token_stream().into_iter())
        .collect(),
      Fields::Unit => Vec::new(),
    };

    let lifetimes = if lifetimes.is_empty() {
      vec![Lifetime::new("data", Span::call_site())]
    } else {
      lifetimes
    };

    Structure {
      ident,
      input,
      lifetimes,
      field_types,
      field_accessors,
    }
  }

  fn field_types(&self) -> &[Type] {
    &self.field_types
  }

  fn field_accessors(&self) -> &[TokenTree] {
    &self.field_accessors
  }

  fn ident(&self) -> &Ident {
    &self.ident
  }

  fn lifetimes(&self) -> &[Lifetime] {
    &self.lifetimes
  }

  fn data_lifetime(&self) -> &Lifetime {
    &self.lifetimes[0]
  }

  fn fixed_size(&self) -> TokenStream {
    let types = self.input.fields.iter().map(|field| field.ty.clone());

    quote!(0#( + <#types as ::data::Data>::FIXED_SIZE)*)
  }

  fn additional_size(&self) -> TokenStream {
    let accessors = self.field_accessors().into_iter();
    quote!(0#( + self.#accessors.additional_size())*)
  }

  fn load_at(&self) -> TokenStream {
    match &self.input.fields {
      Fields::Named(_) => {
        let ident = self.ident();
        let names = self.field_accessors();
        let types = self.field_types();

        quote!(
          let mut offset = offset;

          #(
          let #names = <#types as ::data::Data>::load_at(buffer, offset)?;
          offset += <#types as ::data::Data>::FIXED_SIZE;
          )*

          Ok(#ident {
            #(#names),*
          })
        )
      }
      Fields::Unnamed(_) => {
        let ident = self.ident();
        let variables = self
          .field_types()
          .iter()
          .enumerate()
          .map(|(index, _type)| format_ident!("field{}", index))
          .collect::<Vec<Ident>>();
        let types = self.field_types();

        quote!(
          let mut offset = offset;

          #(
          let #variables = <#types as ::data::Data>::load_at(buffer, offset)?;
          offset += <#types as ::data::Data>::FIXED_SIZE;
          )*

          Ok(#ident (
            #(#variables),*
          ))
        )
      }
      Fields::Unit => {
        let ident = self.ident();
        quote!(Ok(#ident))
      }
    }
  }

  fn store_at(&self) -> TokenStream {
    let accessors = self.field_accessors();
    let types = self.field_types();

    quote!(
      let mut offset: usize = 0;

      #(
      let fixed_size = <#types as ::data::Data>::FIXED_SIZE;
      self.#accessors.store_at(&mut allocation[offset..offset+fixed_size], allocator)?;
      offset += fixed_size;
      )*

      Ok(())
    )
  }
}

impl Tokens for Structure {
  fn tokens(self) -> TokenStream {
    let ident = self.ident.clone();

    let fixed_size = self.fixed_size();

    let additional_size = self.additional_size();

    let store_at = self.store_at();

    let load_at = self.load_at();

    let lifetimes = self.lifetimes();

    let data_lifetime = self.data_lifetime();

    quote!(
      impl<#(#lifetimes),*> ::data::Data<#data_lifetime> for #ident<#(#lifetimes),*> {
        const FIXED_SIZE: usize = #fixed_size;

        fn additional_size(&self) -> usize {
          #additional_size
        }

        fn load_at(buffer: &'a [u8], offset: usize) -> ::data::Result<Self> {
          #load_at
        }

        fn store_at(&self, allocation: &mut [u8], allocator: &mut ::data::Allocator) -> ::data::Result<()> {
          #store_at
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

        fn load_at(buffer: &'a [u8], offset: u64) -> ::data::Result<Self> {
          todo!();
        }

        fn store_at(&self, allocation: &mut [u8], allocator: &mut Allocator) -> ::data::Result<()> {
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
