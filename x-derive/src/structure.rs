use crate::common::*;

use heck::CamelCase;

pub(crate) struct Structure {
  field_accessors: Vec<TokenTree>,
  field_types:     Vec<Type>,
  ident:           Ident,
  input:           DataStruct,
  serializer:      Ident,
  x:               TokenStream,
}

impl Structure {
  pub(crate) fn new(x: TokenStream, ident: Ident, input: DataStruct) -> Self {
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

    let serializer = {
      let mut name = ident.to_string();
      name.push_str("Serializer");
      Ident::new(&name, Span::call_site())
    };

    Self {
      field_accessors,
      field_types,
      ident,
      input,
      serializer,
      x,
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
}

impl Tokens for Structure {
  fn tokens(self) -> TokenStream {
    let ident = &self.ident;

    let types = self.field_types();

    let accessors = self.field_accessors();

    let x = &self.x;

    let body = match &self.input.fields {
      Fields::Named(_) => quote!({#(#accessors: <#types as #x::X>::View,)*}),
      Fields::Unnamed(_) => quote!((#(<#types as #x::X>::View,)*);),
      Fields::Unit => quote!(;),
    };

    let view = {
      let mut name = ident.to_string();
      name.push_str("View");
      Ident::new(&name, Span::call_site())
    };

    let from_constructor = match &self.input.fields {
      Fields::Named(fields) => quote!({#(#accessors: value.#accessors.into(),)*}),
      Fields::Unnamed(fields) => quote!((#(value.#accessors.into(),)*)),
      Fields::Unit => quote!(),
    };

    let serializer = &self.serializer;

    quote!(
      impl #x::X for #ident {
        type View = #view;
        type Serializer<A: #x::Allocator, C: #x::Continuation<A>> = #serializer<A, C>;
      }

      struct #view #body

      impl From<&#view> for #ident {
        fn from(value: &#view) -> Self {
          #ident #from_constructor
        }
      }

      struct #serializer<A: Allocator, C: Continuation<A>> {
        allocator: A,
        #[allow(unused)]
        continuation: #x::core::marker::PhantomData<C>,
      }

      impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for #serializer<A, C> {
        type Native = #ident;

        fn new(allocator: A) -> Self {
          Self {
            continuation: #x::core::marker::PhantomData,
            allocator,
          }
        }

        fn serialize<B: #x::core::borrow::Borrow<Self::Native>>(self, native: B) -> C {
          todo!()
        }
      }
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unit_derive() {
    assert_derive_x_expansion_eq!(
      struct Foo;,

      impl ::x::X for Foo {
        type View = FooView;

        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;
      }

      struct FooView;

      impl From<&FooView> for Foo {
        fn from(value: &FooView) -> Self {
          Foo
        }
      }

      struct FooSerializer<A: Allocator, C: Continuation<A>> {
        allocator: A,
        #[allow(unused)]
        continuation: ::x::core::marker::PhantomData<C>,
      }

      impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for FooSerializer<A, C> {
        type Native = Foo;

        fn new(allocator: A) -> Self {
          Self {
            continuation: ::x::core::marker::PhantomData,
            allocator,
          }
        }

        fn serialize<B: ::x::core::borrow::Borrow<Self::Native>>(self, native: B) -> C {
          C::continuation(self.allocator)
        }
      }
    );
  }

  #[test]
  fn record_derive() {
    assert_derive_x_expansion_eq!(
      struct Foo {
        a: u16,
        b: String,
      },

      impl ::x::X for Foo {
        type View = FooView;

        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;
      }

      struct FooView {
        a: <u16 as ::x::X>::View,
        b: <String as ::x::X>::View,
      }

      impl From<&FooView> for Foo {
        fn from(value: &FooView) -> Self {
          Foo {
            a: value.a.into(),
            b: value.b.into(),
          }
        }
      }

      struct FooSerializer<A: Allocator, C: Continuation<A>> {
        allocator: A,
        #[allow(unused)]
        continuation: ::x::core::marker::PhantomData<C>,
      }

      impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for FooSerializer<A, C> {
        type Native = Foo;

        fn new(allocator: A) -> Self {
          Self {
            continuation: ::x::core::marker::PhantomData,
            allocator,
          }
        }

        fn serialize<B: ::x::core::borrow::Borrow<Self::Native>>(self, native: B) -> C {
          let native = native.borrow();

          self
            .a_begin()
            .serialize(&native.a)
            .b_begin()
            .serialize(&native.b)
        }
      }
    );
  }

  #[test]
  fn tuple_derive() {
    assert_derive_x_expansion_eq!(
      struct Foo(u16, String);,

      impl ::x::X for Foo {
        type View = FooView;

        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;
      }

      struct FooView(<u16 as ::x::X>::View, <String as ::x::X>::View,);

      impl From<&FooView> for Foo {
        fn from(value: &FooView) -> Self {
          Foo(value.0.into(), value.1.into(),)
        }
      }

      struct FooSerializer<A: Allocator, C: Continuation<A>> {
        allocator: A,
        #[allow(unused)]
        continuation: ::x::core::marker::PhantomData<C>,
      }

      impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for FooSerializer<A, C> {
        type Native = Foo;

        fn new(allocator: A) -> Self {
          Self {
            continuation: ::x::core::marker::PhantomData,
            allocator,
          }
        }

        fn serialize<B: ::x::core::borrow::Borrow<Self::Native>>(self, native: B) -> C {
          let native = native.borrow();

          self
            .zero_begin()
            .serialize(&native.a)
            .one_begin()
            .serialize(&native.b)
        }
      }
    );
  }
}
