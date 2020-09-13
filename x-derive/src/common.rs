pub(crate) use std::ffi::OsStr;

pub(crate) use proc_macro2::{Span, TokenStream, TokenTree};
pub(crate) use quote::{format_ident, quote, ToTokens};
pub(crate) use syn::{DataStruct, DeriveInput, Fields, File, Ident, Lifetime, Type};

pub(crate) use crate::tokens::Tokens;

pub(crate) use crate::{error::Error, structure::Structure};
