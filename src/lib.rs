#![no_std]
#![feature(generic_associated_types)]
#![allow(unused)]
#![allow(incomplete_features)]

use common::*;

mod allocator;
mod common;
mod continuation;
mod done;
mod serializer;
mod slice_allocator;
mod u16;
mod view;
mod x;

mod foo;

#[doc(hidden)]
/// This export is used by `x-derive` to access `core`
pub use core;
