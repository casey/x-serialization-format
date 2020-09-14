use crate::common::*;

pub trait View: Sized {
  type Native: X;

  fn to_native(&self) -> Self::Native;

  fn load(bytes: &[u8]) -> &Self {
    // TODO: actually check bytes!
    unsafe { core::mem::transmute(&*bytes.as_ptr()) }
  }
}
