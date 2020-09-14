use crate::common::*;

use heck::CamelCase;

pub(crate) struct Structure {
  field_accessors:   Vec<TokenTree>,
  field_types:       Vec<Type>,
  ident:             Ident,
  input:             DataStruct,
  serialize_methods: Vec<Ident>,
  serializer:        Ident,
  x:                 TokenStream,
}

fn number(index: usize) -> &'static str {
  match index {
    0 => "zero",
    1 => "one",
    2 => "two",
    3 => "three",
    _ => todo!(),
  }
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

    let serialize_methods = match &input.fields {
      Fields::Named(fields) => fields
        .named
        .iter()
        .map(|field| {
          let mut name = field.ident.as_ref().unwrap().to_string();
          name.push_str("_serializer");
          Ident::new(&name, Span::call_site())
        })
        .collect(),
      Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, _)| {
          let name = format!("{}_serializer", number(index));
          Ident::new(&name, Span::call_site())
        })
        .collect(),
      Fields::Unit => Vec::new(),
    };

    Self {
      field_accessors,
      field_types,
      ident,
      input,
      serializer,
      serialize_methods,
      x,
    }
  }

  fn field_count(&self) -> usize {
    self.field_types.len()
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

  fn serialize_methods(&self) -> &[Ident] {
    &self.serialize_methods
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

    let serialize_methods = self.serialize_methods();

    let serialize_inner = if self.field_count() == 0 {
      quote!(C::continuation(self.allocator))
    } else {
      quote!(
        let native = native.borrow();
        self #(.#serialize_methods().serialize(&native.#accessors))*
      )
    };

    let to_native_inner = match &self.input.fields {
      Fields::Named(fields) => quote!({#(#accessors: self.#accessors.to_native(),)*}),
      Fields::Unnamed(fields) => quote!((#(self.#accessors.to_native(),)*)),
      Fields::Unit => quote!(),
    };

    quote!(
      impl #x::X for #ident {
        type View = #view;
        type Serializer<A: #x::Allocator, C: #x::Continuation<A>> = #serializer<A, C>;
      }

      struct #view #body

      impl View for #view {
        type Native = #ident;

        fn to_native(&self) -> Self::Native {
          #ident #to_native_inner
        }
      }

      impl From<&#view> for #ident {
        fn from(view: &#view) -> Self {
          view.to_native()
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
          #serialize_inner
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

      impl View for FooView {
        type Native = Foo;

        fn to_native(&self) -> Self::Native {
          Foo
        }
      }

      impl From<&FooView> for Foo {
        fn from(view: &FooView) -> Self {
          view.to_native()
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

      impl View for FooView {
        type Native = Foo;

        fn to_native(&self) -> Self::Native {
          Foo {
            a: self.a.to_native(),
            b: self.b.to_native(),
          }
        }
      }

      impl From<&FooView> for Foo {
        fn from(view: &FooView) -> Self {
          view.to_native()
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
            .a_serializer()
            .serialize(&native.a)
            .b_serializer()
            .serialize(&native.b)
        }
      }

      impl<A: Allocator, C: Continuation<A>> FooSerializer<A, C> {
        fn a(self, value: u16) -> FooSerializerB<A, C> {
          self.a_serializer().serialize(value)
        }

        fn a_serializer(self) -> <u16 as X>::Serializer<A, FooSerializerB<A, C>> {
          <u16 as X>::Serializer::new(self.0)
        }
      }

      struct FooSerializerB<A: Allocator, C: Continuation<A>> {
        allocator: A,
        #[allow(unused)]
        continuation: ::x::core::marker::PhantomData<C>,
      }

      impl<A: Allocator, C: Continuation<A>> FooSerializerB<A, C> {
        fn b(self, value: String) -> C {
          self.one_serializer().serialize(value)
        }

        fn b_serializer(self) -> <String as X>::Serializer<A, C> {
          <String as X>::Serializer::new(self.allocator)
        }
      }

      impl<A: Allocator, C: Continuation<A>> Continuation<A> for FooSerializerB<A, C> {
        fn continuation(allocator: A) -> Self {
          FooSerializerOne {
            continuation: ::x::core::marker::PhantomData,
            allocator,
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

        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;
      }

      struct FooView(<u16 as ::x::X>::View, <String as ::x::X>::View,);

      impl View for FooView {
        type Native = Foo;

        fn to_native(&self) -> Self::Native {
          Foo(self.0.to_native(), self.1.to_native(),)
        }
      }

      impl From<&FooView> for Foo {
        fn from(view: &FooView) -> Self {
          view.to_native()
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
            .zero_serializer()
            .serialize(&native.0)
            .one_serializer()
            .serialize(&native.1)
        }
      }

      impl<A: Allocator, C: Continuation<A>> FooSerializer<A, C> {
        fn zero(self, value: u16) -> FooSerializerOne<A, C> {
          self.zero_serializer().serialize(value)
        }

        fn zero_serializer(self) -> <u16 as X>::Serializer<A, FooSerializerOne<A, C>> {
          <u16 as X>::Serializer::new(self.0)
        }
      }

      struct FooSerializerOne<A: Allocator, C: Continuation<A>> {
        allocator: A,
        #[allow(unused)]
        continuation: ::x::core::marker::PhantomData<C>,
      }

      impl<A: Allocator, C: Continuation<A>> FooSerializerOne<A, C> {
        fn one(self, value: String) -> C {
          self.one_serializer().serialize(value)
        }

        fn one_serializer(self) -> <String as X>::Serializer<A, C> {
          <String as X>::Serializer::new(self.allocator)
        }
      }

      impl<A: Allocator, C: Continuation<A>> Continuation<A> for FooSerializerOne<A, C> {
        fn continuation(allocator: A) -> Self {
          FooSerializerOne {
            continuation: ::x::core::marker::PhantomData,
            allocator,
          }
        }
      }
    );
  }
}
