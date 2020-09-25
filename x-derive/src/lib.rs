use crate::common::*;

#[cfg(test)]
#[macro_use]
mod test;

mod common;
mod error;
mod input_attributes;
mod structure;
mod tokens;

// #[derive(X)]
// #[x(derive_from_view)]
// struct Foo {
//   #[x(0)]
//   bar: u32,
//   #[x(1)]
//   baz: u8,
// }

#[derive(FromDeriveInput)]
struct Input {
  // ident:      Ident,
  #[darling(default)]
  attributes: InputAttributes,
  // data:       Data,
}

type Data = darling::ast::Data<Variant, Field>;

#[derive(FromField)]
struct Field {
  ident: Option<Ident>,
  ty:    Type,
}

#[derive(FromVariant)]
struct Variant;

#[proc_macro_derive(X, attributes(x))]
pub fn derive_x(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  derive_x_inner(item.into()).tokens().into()
}

fn derive_x_inner(input: TokenStream) -> Result<TokenStream, Error> {
  let derive_input = syn::parse2::<DeriveInput>(input)?;

  let input = Input::from_derive_input(&derive_input)?;

  let pkg = std::env::var_os("CARGO_PKG_NAME")
    .map(|pkg| pkg.to_string_lossy().into_owned())
    .unwrap_or_default();

  let x = {
    if pkg == "x" || pkg == "x-derive" {
      quote!(::x)
    } else {
      let name = match proc_macro_crate::crate_name("x") {
        Ok(name) => name,
        Err(err) => panic!(format!(
          "The `x-derive` crate requires that the `x` crate be present as a dependency: {}",
          err
        )),
      };

      let ident = Ident::new(&name, Span::call_site());
      quote!(::#ident)
    }
  };

  match derive_input.data {
    syn::Data::Struct(data) =>
      Ok(Structure::new(x, derive_input.ident, input.attributes, data).tokens()),
    syn::Data::Enum(_) => todo!(),
    syn::Data::Union(_) => Err(Error::Union),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn union_error() {
    assert_derive_x_error_match!(union Foo{}, Error::Union);
  }
}
