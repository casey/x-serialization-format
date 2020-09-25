pub(crate) use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
pub(crate) use proc_macro2::{Span, TokenStream, TokenTree};
pub(crate) use quote::{format_ident, quote, ToTokens};
pub(crate) use syn::{DataStruct, DeriveInput, Fields, Ident, Type};

pub(crate) use crate::tokens::Tokens;

pub(crate) use crate::{error::Error, input_attributes::InputAttributes, structure::Structure};

#[cfg(test)]
mod test {
  pub(crate) use syn::File;
}

#[cfg(test)]
pub(crate) use test::*;
