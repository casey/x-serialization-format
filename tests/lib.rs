#![no_std]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

use x::{Serializer, View, X};

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
  let want = &[1, 2, 3, 4, 5, 6, 7, 8];

  // THE BIG PAY OFF. CHECK OUT HOW FUCKING CLEAN IT IS. ANY DEVIATION FROM
  // CANONICAL ORDERING IS SWIFTLY PUNISHED WITH BRUTAL TYPE ERRORS.
  let have = Record::store_to_vec()
    .a(513)
    .b(1027)
    .c(Tuple(1541, 2055))
    .d(Unit)
    .done();

  assert_eq!(have, want);

  // Alternatively:
  let have = Record::store_to_vec()
    .a(513)
    .b(1027)
    .c_serializer()
    .zero(1541)
    .one(2055)
    .d(Unit)
    .done();

  assert_eq!(have, want);

  // Alternatively:
  let have = Record::store_to_vec()
    .serialize(Record {
      a: 513,
      b: 1027,
      c: Tuple(1541, 2055),
      d: Unit,
    })
    .done();

  assert_eq!(have, want);

  let foo = RecordView::load(&have);

  assert_eq!(foo.a(), 513);
  assert_eq!(foo.b(), 1027);
  assert_eq!(foo.c.zero(), 1541);
  assert_eq!(foo.c.one(), 2055);
  assert_eq!(foo.c(), Tuple(1541, 2055));
  assert_eq!(foo.d(), Unit);
}
