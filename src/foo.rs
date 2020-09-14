use crate::common::*;

#[derive(X)]
struct Foo {
  a: u16,
  b: u16,
  c: Bar,
  d: Baz,
}

#[derive(X, PartialEq, Debug)]
struct Bar(u16, u16);

#[derive(X, PartialEq, Debug)]
struct Baz;

#[test]
fn construct() {
  let buffer = &mut [0, 0, 0, 0, 0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  /// THE BIG PAY OFF. CHECK OUT HOW FUCKING CLEAN IT IS. ANY DEVIATION FROM
  /// CANONICAL ORDERING IS SWIFTLY PUNISHED WITH BRUTAL TYPE ERRORS.
  Foo::store(allocator)
    .a(513)
    .b(1027)
    .c(Bar(1541, 2055))
    .d(Baz)
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);

  /// Alternatively:
  let buffer = &mut [0, 0, 0, 0, 0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  Foo::store(allocator)
    .a(513)
    .b(1027)
    .c_serializer()
    .zero(1541)
    .one(2055)
    .d(Baz)
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);

  /// Alternatively:
  let buffer = &mut [0, 0, 0, 0, 0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  Foo::store(allocator)
    .serialize(Foo {
      a: 513,
      b: 1027,
      c: Bar(1541, 2055),
      d: Baz,
    })
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);

  let foo = unsafe { core::mem::transmute::<&[u8; 8], &FooView>(buffer) };

  assert_eq!(foo.a(), 513);
  assert_eq!(foo.b(), 1027);
  assert_eq!(foo.c.zero(), 1541);
  assert_eq!(foo.c.one(), 2055);
  assert_eq!(foo.c(), Bar(1541, 2055));
  assert_eq!(foo.d(), Baz);
}
