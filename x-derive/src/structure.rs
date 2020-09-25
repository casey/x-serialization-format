use crate::common::*;

use heck::CamelCase;

// TODO:
// - getters that return reference to field
// - getters that return to_native for field

pub(crate) struct Structure {
  field_accessors:    Vec<TokenTree>,
  field_methods:      Vec<Ident>,
  field_types:        Vec<Type>,
  ident:              Ident,
  input:              DataStruct,
  serializer_methods: Vec<Ident>,
  serializers:        Vec<Ident>,
  krate:              TokenStream,
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
  pub(crate) fn new(
    krate: TokenStream,
    ident: Ident,
    attributes: InputAttributes,
    input: DataStruct,
  ) -> Self {
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

    let serializer = |index: usize, field_name: Option<&str>| {
      if index == 0 {
        format_ident!("{}Serializer", ident)
      } else {
        format_ident!("{}Serializer{}", ident, field_name.unwrap().to_camel_case())
      }
    };

    let serializers = match &input.fields {
      Fields::Named(fields) => fields
        .named
        .iter()
        .enumerate()
        .map(|(index, field)| serializer(index, Some(&field.ident.as_ref().unwrap().to_string())))
        .collect(),
      Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, _)| serializer(index, Some(&number(index))))
        .collect(),
      Fields::Unit => vec![serializer(0, None)],
    };

    let serializer_methods = match &input.fields {
      Fields::Named(fields) => fields
        .named
        .iter()
        .map(|field| format_ident!("{}_serializer", field.ident.as_ref().unwrap()))
        .collect(),
      Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, _)| format_ident!("{}_serializer", number(index)))
        .collect(),
      Fields::Unit => Vec::new(),
    };

    let field_methods = match &input.fields {
      Fields::Named(fields) => fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap().clone())
        .collect(),
      Fields::Unnamed(fields) => fields
        .unnamed
        .iter()
        .enumerate()
        .map(|(index, _)| {
          let name = number(index);
          Ident::new(&name, Span::call_site())
        })
        .collect(),
      Fields::Unit => Vec::new(),
    };

    Self {
      field_accessors,
      field_methods,
      field_types,
      ident,
      input,
      serializers,
      serializer_methods,
      krate,
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

  fn serializer_methods(&self) -> &[Ident] {
    &self.serializer_methods
  }
}

impl Tokens for Structure {
  fn tokens(self) -> TokenStream {
    let ident = &self.ident;

    let types = self.field_types();

    let accessors = self.field_accessors();

    let x = &self.krate;

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

    let serializers = &self.serializers;

    let first_serializer = &self.serializers[0];

    let serializer_methods = self.serializer_methods();

    let field_methods = &self.field_methods;

    let continuable = &self.serializers[1..];

    let serialize_inner = if self.field_count() == 0 {
      quote!(serializer.state.continuation())
    } else {
      quote!(
        serializer #(.#field_methods(&self.#accessors))*
      )
    };

    // let from_view_inner = match &self.input.fields {
    //   Fields::Named(_) => quote!({#(#accessors: view.#field_methods(),)*}),
    //   Fields::Unnamed(_) => quote!((#(view.#field_methods(),)*)),
    //   Fields::Unit => quote!(),
    // };

    let continuations = (0..serializers.len()).into_iter().map(|i| {
      if let Some(serializer) = serializers.get(i + 1) {
        quote!(#serializer<A, C>)
      } else {
        quote!(C)
      }
    });

    // let view_getters = match &self.input.fields {
    //   Fields::Named(_) | Fields::Unnamed(_) => quote!(
    //     impl #view {
    //       #(
    //       fn #field_methods(&self) -> #types {
    //         #x::X::from_view(&self.#accessors)
    //       }
    //       )*
    //     }
    //   ),
    //   Fields::Unit => quote!(),
    // };

    quote!(
      impl #x::X for #ident {
        type View = #view;

        fn serialize<A: #x::Allocator, C: #x::Continuation<A>>(&self, serializer: Self::Serializer<A, C>) -> C {
          #serialize_inner
        }
      }

      #[repr(C)]
      struct #view #body

      // #view_getters

      impl #x::View for #view {
        type Serializer<A: #x::Allocator, C: #x::Continuation<A>> = #first_serializer<A, C>;

        fn check<'value>(
          suspect: &'value #x::core::mem::MaybeUninit<Self>,
          buffer: &[u8],
        ) -> #x::Result<&'value Self> {
          let pointer: *const Self = suspect.as_ptr();
          #(
          {
            type FieldView = <#types as #x::X>::View;
            let field_pointer: *const FieldView = unsafe { &raw const (*pointer).#accessors };
            let maybe_uninit_pointer = field_pointer as *const #x::core::mem::MaybeUninit<FieldView>;
            let maybe_uninit_ref = unsafe { &*maybe_uninit_pointer } ;
            FieldView::check(maybe_uninit_ref, buffer)?;
          }
          )*
          // All fields are valid, so the struct is valid.
          Ok(unsafe { suspect.assume_init_ref() })
        }
      }

      #(
      struct #serializers<A: #x::Allocator, C: #x::Continuation<A>> {
        state: #x::State<A, C>,
      }
      )*

      #(
      impl <A: #x::Allocator, C: #x::Continuation<A>> #serializers<A, C> {
        fn #field_methods<N>(self, value: &N) -> #continuations
          where N: #x::X<Serializer = <#types as #x::X>::Serializer<A, #continuations>>,
        {
          self.#serializer_methods().serialize(value)
        }

        fn #serializer_methods(self) -> <#types as #x::X>::Serializer<A, #continuations> {
          <#types as #x::X>::Serializer::new(self.state.identity())
        }
      }
      )*

      impl<A: #x::Allocator, C: #x::Continuation<A>> #x::Serializer<A, C> for #first_serializer<A, C> {
        fn new(state: #x::State<A, C>) -> Self {
          Self { state }
        }
      }

      #(
      impl<A: #x::Allocator, C: #x::Continuation<A>> #x::Continuation<A> for #continuable<A, C> {
        type Seed = C::Seed;

        fn continuation(state: #x::State<A, Self>) -> Self {
          // TODO: Why the fuck is this call to identity necessary?
          #continuable { state: state.identity() }
        }
      }
      )*
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

        fn serialize<A: ::x::Allocator, C: ::x::Continuation<A>>(&self, serializer: Self::Serializer<A, C>) -> C {
          serializer.state.continuation()
        }
      }

      #[repr(C)]
      struct FooView;

      impl ::x::View for FooView {
        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;

        fn check<'value>(
          suspect: &'value ::x::core::mem::MaybeUninit<Self>,
          buffer: &[u8],
        ) -> ::x::Result<&'value Self> {
          let pointer: *const Self = suspect.as_ptr();
          Ok(unsafe { suspect.assume_init_ref() })
        }
      }

      struct FooSerializer<A: ::x::Allocator, C: ::x::Continuation<A>> {
        state: ::x::State<A, C>,
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> ::x::Serializer<A, C> for FooSerializer<A, C> {
        fn new(state: ::x::State<A, C>) -> Self {
          Self { state }
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

        fn serialize<A: ::x::Allocator, C: ::x::Continuation<A>>(&self, serializer: Self::Serializer<A, C>) -> C {
          serializer.a(&self.a).b(&self.b)
        }
      }

      #[repr(C)]
      struct FooView {
        a: <u16 as ::x::X>::View,
        b: <String as ::x::X>::View,
      }

      impl ::x::View for FooView {
        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;

        fn check<'value>(
          suspect: &'value ::x::core::mem::MaybeUninit<Self>,
          buffer: &[u8],
        ) -> ::x::Result<&'value Self> {
          let pointer: *const Self = suspect.as_ptr();
          {
            type FieldView = <u16 as ::x::X>::View;
            let field_pointer: *const FieldView = unsafe { &raw const (*pointer).a };
            let maybe_uninit_pointer = field_pointer as *const ::x::core::mem::MaybeUninit<FieldView>;
            let maybe_uninit_ref = unsafe { &*maybe_uninit_pointer } ;
            FieldView::check(maybe_uninit_ref, buffer)?;
          }
          {
            type FieldView = <String as ::x::X>::View;
            let field_pointer: *const FieldView = unsafe { &raw const (*pointer).b };
            let maybe_uninit_pointer = field_pointer as *const ::x::core::mem::MaybeUninit<FieldView>;
            let maybe_uninit_ref = unsafe { &*maybe_uninit_pointer } ;
            FieldView::check(maybe_uninit_ref, buffer)?;
          }
          Ok(unsafe { suspect.assume_init_ref() })
        }
      }

      struct FooSerializer<A: ::x::Allocator, C: ::x::Continuation<A>> {
        state: ::x::State<A, C>,
      }

      struct FooSerializerB<A: ::x::Allocator, C: ::x::Continuation<A>> {
        state: ::x::State<A, C>,
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> FooSerializer<A, C> {
        fn a<N>(self, value: &N) -> FooSerializerB<A, C>
          where N: ::x::X<Serializer = <u16 as ::x::X>::Serializer<A, C>>,
        {
          self.a_serializer().serialize(value)
        }

        fn a_serializer(self) -> <u16 as ::x::X>::Serializer<A, FooSerializerB<A, C> > {
          <u16 as ::x::X>::Serializer::new(self.state.identity())
        }
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> FooSerializerB<A, C> {
        fn b<N>(self, value: &N) -> C
          where N: ::x::X<Serializer = <String as ::x::X>::Serializer<A, C>>,
        {
          self.b_serializer().serialize(value)
        }

        fn b_serializer(self) -> <String as ::x::X>::Serializer<A, C> {
          <String as ::x::X>::Serializer::new(self.state.identity())
        }
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> ::x::Serializer<A, C> for FooSerializer<A, C> {
        fn new(state: ::x::State<A, C>) -> Self {
          Self { state }
        }
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> ::x::Continuation<A> for FooSerializerB<A, C> {
        type Seed = C::Seed;

        fn continuation(state: ::x::State<A, Self>) -> Self {
          FooSerializerB { state: state.identity() }
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

        fn serialize<A: ::x::Allocator, C: ::x::Continuation<A>>(&self, serializer: Self::Serializer<A, C>) -> C {
          serializer.zero(&self.0).one(&self.1)
        }
      }

      #[repr(C)]
      struct FooView(<u16 as ::x::X>::View, <String as ::x::X>::View,);

      impl ::x::View for FooView {
        type Serializer<A: ::x::Allocator, C: ::x::Continuation<A>> = FooSerializer<A, C>;

        fn check<'value>(
          suspect: &'value ::x::core::mem::MaybeUninit<Self>,
          buffer: &[u8],
        ) -> ::x::Result<&'value Self> {
          let pointer: *const Self = suspect.as_ptr();
          {
            type FieldView = <u16 as ::x::X>::View;
            let field_pointer: *const FieldView = unsafe { &raw const (*pointer).0 };
            let maybe_uninit_pointer = field_pointer as *const ::x::core::mem::MaybeUninit<FieldView>;
            let maybe_uninit_ref = unsafe { &*maybe_uninit_pointer } ;
            FieldView::check(maybe_uninit_ref, buffer)?;
          }
          {
            type FieldView = <String as ::x::X>::View;
            let field_pointer: *const FieldView = unsafe { &raw const (*pointer).1 };
            let maybe_uninit_pointer = field_pointer as *const ::x::core::mem::MaybeUninit<FieldView>;
            let maybe_uninit_ref = unsafe { &*maybe_uninit_pointer } ;
            FieldView::check(maybe_uninit_ref, buffer)?;
          }
          Ok(unsafe { suspect.assume_init_ref() })
        }
      }

      struct FooSerializer<A: ::x::Allocator, C: ::x::Continuation<A>> {
        state: ::x::State<A, C>,
      }

      struct FooSerializerOne<A: ::x::Allocator, C: ::x::Continuation<A>> {
        state: ::x::State<A, C>,
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> FooSerializer<A, C> {
        fn zero<N>(self, value: &N) -> FooSerializerOne<A, C>
          where N: ::x::X<Serializer = <u16 as ::x::X>::Serializer<A, C>>,
        {
          self.zero_serializer().serialize(value)
        }

        fn zero_serializer(self) -> <u16 as ::x::X>::Serializer<A, FooSerializerOne<A, C> > {
          <u16 as ::x::X>::Serializer::new(self.state.identity())
        }
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> FooSerializerOne<A, C> {
        fn one<N>(self, value: &N) -> C
          where N: ::x::X<Serializer = <String as ::x::X>::Serializer<A, C>>,
        {
          self.one_serializer().serialize(value)
        }

        fn one_serializer(self) -> <String as ::x::X>::Serializer<A, C> {
          <String as ::x::X>::Serializer::new(self.state.identity())
        }
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> ::x::Serializer<A, C> for FooSerializer<A, C> {
        fn new(state: ::x::State<A, C>) -> Self {
          Self { state }
        }
      }

      impl<A: ::x::Allocator, C: ::x::Continuation<A>> ::x::Continuation<A> for FooSerializerOne<A, C> {
        type Seed = C::Seed;

        fn continuation(state: ::x::State<A, Self>) -> Self {
          FooSerializerOne { state: state.identity() }
        }
      }
    );
  }
}
