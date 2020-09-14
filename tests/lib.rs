#![no_std]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

use x::{Serializer, SliceAllocator, View, X};

#[derive(X)]
struct Record {
  a: u16,
  b: u16,
  c: Tuple,
  d: Unit,
}

#[derive(X, PartialEq, Debug)]
struct Tuple(u16, u16);

#[derive(X, PartialEq, Debug)]
struct Unit;

#[test]
fn construct() {
  let buffer = &mut [0, 0, 0, 0, 0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  // THE BIG PAY OFF. CHECK OUT HOW FUCKING CLEAN IT IS. ANY DEVIATION FROM
  // CANONICAL ORDERING IS SWIFTLY PUNISHED WITH BRUTAL TYPE ERRORS.
  Record::store(allocator)
    .a(513)
    .b(1027)
    .c(Tuple(1541, 2055))
    .d(Unit)
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);

  // Alternatively:
  let buffer = &mut [0, 0, 0, 0, 0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  Record::store(allocator)
    .a(513)
    .b(1027)
    .c_serializer()
    .zero(1541)
    .one(2055)
    .d(Unit)
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);

  // Alternatively:
  let buffer = &mut [0, 0, 0, 0, 0, 0, 0, 0];
  let allocator = SliceAllocator::new(buffer);

  Record::store(allocator)
    .serialize(Record {
      a: 513,
      b: 1027,
      c: Tuple(1541, 2055),
      d: Unit,
    })
    .done();

  assert_eq!(buffer, &[1, 2, 3, 4, 5, 6, 7, 8]);

  let foo = unsafe { core::mem::transmute::<&[u8; 8], &RecordView>(buffer) };

  assert_eq!(foo.a(), 513);
  assert_eq!(foo.b(), 1027);
  assert_eq!(foo.c.zero(), 1541);
  assert_eq!(foo.c.one(), 2055);
  assert_eq!(foo.c(), Tuple(1541, 2055));
  assert_eq!(foo.d(), Unit);
}
