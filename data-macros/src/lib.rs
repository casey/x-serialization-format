use crate::common::*;

#[cfg(test)]
#[macro_use]
mod test;

mod common;
mod error;
mod structure;
mod tokens;

#[proc_macro_derive(Data)]
pub fn derive_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  derive_data_inner(item.into()).tokens().into()
}

fn derive_data_inner(input: TokenStream) -> Result<TokenStream, Error> {
  let input = syn::parse2::<DeriveInput>(input)?;

  let lifetimes = input
    .generics
    .lifetimes()
    .map(|def| def.lifetime.clone())
    .collect();

  match input.data {
    syn::Data::Struct(data) => Ok(Structure::new(input.ident, lifetimes, data).tokens()),
    syn::Data::Enum(_) => todo!(),
    syn::Data::Union(_) => Err(Error::Union),
  }
}
