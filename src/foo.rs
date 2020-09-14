use crate::common::*;

#[derive(X)]
// #[x(struct = FooStruct)]
struct Foo {
  a: u16,
  b: u16,
}

#[repr(C)]
struct FooStruct {
  a: <u16 as X>::View,
  b: <u16 as X>::View,
}

impl From<&FooStruct> for Foo {
  fn from(value: &FooStruct) -> Self {
    value.to_native()
  }
}

impl View for FooStruct {
  type Native = Foo;

  fn to_native(&self) -> Self::Native {
    Foo {
      a: self.a.to_native(),
      b: self.b.to_native(),
    }
  }
}

struct FooA<A: Allocator, C: Continuation<A>>(A, PhantomData<C>);

impl<A: Allocator, C: Continuation<A>> Serializer<A, C> for FooA<A, C> {
  type Native = Foo;

  fn new(allocator: A) -> Self {
    FooA(allocator, PhantomData)
  }

  fn serialize<B: Borrow<Self::Native>>(self, native: B) -> C {
    let native = native.borrow();

    self
      .a_serializer()
      .serialize(&native.a)
      .b_serializer()
      .serialize(&native.b)
  }
}

impl<A: Allocator, C: Continuation<A>> FooA<A, C> {
  fn a_serializer(self) -> <u16 as X>::Serializer<A, FooB<A, C>> {
    <u16 as X>::Serializer::new(self.0)
  }

  fn a(self, value: u16) -> FooB<A, C> {
    self.a_serializer().serialize(&value)
  }
}

struct FooB<A: Allocator, C: Continuation<A>>(A, PhantomData<C>);

impl<A: Allocator, C: Continuation<A>> FooB<A, C> {
  fn b(self, value: u16) -> C {
    self.b_serializer().serialize(&value)
  }

  fn b_serializer(self) -> <u16 as X>::Serializer<A, C> {
    <u16 as X>::Serializer::new(self.0)
  }
}

impl<A: Allocator, C: Continuation<A>> Continuation<A> for FooB<A, C> {
  fn continuation(allocator: A) -> Self {
    FooB(allocator, PhantomData)
  }
}

#[test]
fn construct() {
  let buffer = &mut [0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  /// THE BIG PAY OFF. CHECK OUT HOW FUCKING CLEAN IT IS. ANY DEVIATION FROM
  /// CANONICAL ORDERING IS SWIFTLY PUNISHED WITH BRUTAL TYPE ERRORS.
  Foo::store(allocator).a(513).b(1027).done();

  assert_eq!(buffer, &[1, 2, 3, 4]);

  /// Alternatively:
  let buffer = &mut [0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  Foo::store(allocator)
    .serialize(Foo { a: 513, b: 1027 })
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4]);
}
