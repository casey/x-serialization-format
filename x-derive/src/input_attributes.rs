use crate::common::*;

#[derive(Default, FromMeta)]
#[darling(default)]
pub(crate) struct InputAttributes {
  derive_from_view: bool,
}
