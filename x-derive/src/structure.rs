use crate::common::*;

use heck::CamelCase;

pub(crate) struct Structure {
  field_accessors: Vec<TokenTree>,
  field_types:     Vec<Type>,
  ident:           Ident,
  input:           DataStruct,
  serializers:     Vec<Ident>,
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

    let serializers = match &input.fields {
      Fields::Named(fields) => fields
        .named
        .iter()
        .map(|field| {
          let mut name = ident.to_string();
          name.push_str(
            &field
              .ident
              .as_ref()
              .expect("record structs fields always have idents")
              .to_string()
              .to_camel_case(),
          );
          Ident::new(&name, Span::call_site())
        })
        .collect(),
      Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, _)| {
          let mut name = ident.to_string();
          name.push_str(match index {
            0 => "Zero",
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            _ => panic!(
              "Please open a pull request on https://github.com/casey/x requesting that larger \
               tuple structs be supported"
            ),
          });
          Ident::new(&name, Span::call_site())
        })
        .collect(),
      Fields::Unit => {
        let mut name = ident.to_string();
        name.push_str("Serializer");
        vec![Ident::new(&name, Span::call_site())]
      },
    };

    Self {
      field_accessors,
      field_types,
      ident,
      input,
      serializers,
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

    let initial_serializer = &self.serializers[0];

    quote!(
      impl #x::X for #ident {
        type View = #view;
        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = #initial_serializer<A, C>;
      }

      struct #view #body

      impl From<&#view> for #ident {
        fn from(value: &#view) -> Self {
          #ident #from_constructor
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

        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooA<A, C>;
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
    );
  }

  #[test]
  fn tuple_derive() {
    assert_derive_x_expansion_eq!(
      struct Foo(u16, String);,

      impl ::x::X for Foo {
        type View = FooView;

        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooZero<A, C>;
      }

      struct FooView(<u16 as ::x::X>::View, <String as ::x::X>::View,);

      impl From<&FooView> for Foo {
        fn from(value: &FooView) -> Self {
          Foo(value.0.into(), value.1.into(),)
        }
      }
    );
  }
}
