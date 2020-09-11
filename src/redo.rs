#![feature(generic_associated_types)]
#![allow(unused)]
#![allow(incomplete_features)]
// struct
// table
// slice
// string
// enum
// serialize
// deerialize

trait X {
  type View;
  type Builder<A: Allocator>: Builder<A>;
}

impl X for u16 {
  type Builder<A: Allocator> = U16Builder<A>;
  type View = U16;
}

// #[derive(X)]
// #[x(struct)]
struct Foo {
  a: u16,
  /* b: Bar,
   * c: u16, */
}

// todo:
// - assert all bytes were written to
// - can allocate to a stream with some amount of buffering?

// problem:
// - would like builders to contain typed allocations, e.g. &mut Foo vs &mut
//   [u8; 4]

// builders are just wrappers around allocators

#[repr(C)]
struct FooStruct {
  a: <u16 as X>::View,
  /* b: <Bar as X>::View,
   * c: <u16 as X>::View, */
}

trait Allocator {
  fn push(&mut self, size: usize);

  fn pop(&mut self);

  fn write(&mut self, bytes: &[u8]);
}

trait Builder<A: Allocator> {
  fn new(allocator: A) -> Self;

  fn done(self) -> A;
}

use core::mem::size_of;

struct FooA<A: Allocator>(A);

impl<A: Allocator> FooA<A> {
  fn a(self) -> <u16 as X<A>>::Builder {
    <u16 as X>::Builder::new(self.0)
  }
}

struct FooB<A: Allocator>(A);

// struct Bar {}

// struct BarStruct {}

// struct BarBuilder {}

struct U16Builder<A: Allocator>(A);

impl<A: Allocator> Builder<A> for U16Builder<A> {
  fn new(allocator: A) -> Self {
    U16Builder(allocator)
  }

  fn done(self) -> A {
    self.0
  }
}

// impl X for Bar {
//   type Builder<A: Allocator> = BarBuilder<A>;
//   type View = BarStruct;
// }

// impl X for Foo {
//   type Builder<A: Allocator> = FooBuilderA<A>;
//   type View = FooStruct;
// }

struct U16 {
  bytes: [u8; 2],
}
