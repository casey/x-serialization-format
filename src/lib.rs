#![no_std]
#![feature(generic_associated_types)]
#![allow(unused)]
#![allow(incomplete_features)]

use common::*;

mod allocator;
mod common;
mod continuation;
mod done;
mod into_allocator;
mod serializer;
mod slice_allocator;
mod u16;
mod x;

mod foo;
