use crate::common::*;

#[derive(X)]
struct Foo {
  a: u16,
  b: u16,
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
