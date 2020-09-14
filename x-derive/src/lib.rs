use crate::common::*;

#[cfg(test)]
#[macro_use]
mod test;

mod common;
mod error;
mod structure;
mod tokens;

#[proc_macro_derive(X, attributes(x))]
pub fn derive_x(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  derive_x_inner(item.into()).tokens().into()
}

fn derive_x_inner(input: TokenStream) -> Result<TokenStream, Error> {
  let input = syn::parse2::<DeriveInput>(input)?;

  let pkg = std::env::var_os("CARGO_PKG_NAME")
    .map(|pkg| pkg.to_string_lossy().into_owned())
    .unwrap_or(String::from(""));

  let x = {
    if pkg == "x" {
      quote!(::x)
    } else if pkg == "x-derive" {
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

  match input.data {
    syn::Data::Struct(data) => Ok(Structure::new(x, input.ident, data).tokens()),
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
