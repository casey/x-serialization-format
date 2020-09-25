use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  Syn(syn::Error),
  Darling(darling::Error),
  Union,
}

impl From<syn::Error> for Error {
  fn from(error: syn::Error) -> Error {
    Error::Syn(error)
  }
}

impl From<darling::Error> for Error {
  fn from(error: darling::Error) -> Error {
    Error::Darling(error)
  }
}

impl Tokens for Error {
  fn tokens(self) -> TokenStream {
    match self {
      Error::Syn(error) => error.to_compile_error(),
      Error::Union => todo!(),
      Error::Darling(error) => error.write_errors(),
    }
  }
}
