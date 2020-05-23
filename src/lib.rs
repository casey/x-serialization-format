#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

mod allocator;
mod common;
mod error;
mod offset;
mod primitive;
mod range_ext;
mod result;
mod size;
mod slice;
mod slice_ext;
mod str;
mod u64;
mod u8;
mod unit;
mod value;
mod view;

#[cfg(test)]
mod test;

pub use crate::{error::Error, result::Result, view::View};
