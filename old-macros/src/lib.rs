use crate::common::*;

#[cfg(test)]
#[macro_use]
mod test;

mod attribute;
mod common;
mod error;
mod structure;
mod table;
mod tokens;

#[proc_macro_attribute]
pub fn table(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  Table::attribute(attr, item)
}

#[proc_macro_derive(Data)]
pub fn derive_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  derive_data_inner(item.into()).tokens().into()
}

fn derive_data_inner(input: TokenStream) -> Result<TokenStream, Error> {
  let input = syn::parse2::<DeriveInput>(input)?;

  match input.data {
    syn::Data::Struct(data) => Ok(Structure::new(input.ident, data).tokens()),
    syn::Data::Enum(_) => todo!(),
    syn::Data::Union(_) => Err(Error::Union),
  }
}
