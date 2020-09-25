#![no_std]
#![feature(generic_associated_types)]
#![feature(raw_ref_op)]
#![feature(maybe_uninit_ref)]
#![allow(incomplete_features)]

use x::{Serializer, View, X};

#[derive(X)]
struct Record {
  a: u16,
  b: u16,
  c: Tuple,
  d: Unit,
  e: i32,
  f: bool,
  g: u8,
}

#[derive(X, PartialEq, Debug)]
struct Tuple(u16, u16);

#[derive(X, PartialEq, Debug)]
struct Unit;

#[test]
fn construct() {
  let want = &[1, 2, 3, 4, 5, 6, 7, 8, 0xFE, 0xFF, 0xFF, 0xFF, 0x01, 0x7F];

  // THE BIG PAY OFF. CHECK OUT HOW FUCKING CLEAN IT IS. ANY DEVIATION FROM
  // CANONICAL ORDERING IS SWIFTLY PUNISHED WITH BRUTAL TYPE ERRORS.
  let have = Record::store_to_vec()
    .a(&513u16)
    .b(&1027u16)
    .c(&Tuple(1541u16, 2055u16))
    .d(&Unit)
    .e(&-2i32)
    .f(&true)
    .g(&127u8)
    .done();

  assert_eq!(have, want);

  // Alternatively:
  let have = Record::store_to_vec()
    .a(&513u16)
    .b(&1027u16)
    .c_serializer()
    .zero(&1541u16)
    .one(&2055u16)
    .d(&Unit)
    .e(&-2i32)
    .f(&true)
    .g(&127u8)
    .done();

  assert_eq!(have, want);

  // Alternatively:
  let have = Record::store_to_vec()
    .serialize(&Record {
      a: 513u16,
      b: 1027u16,
      c: Tuple(1541u16, 2055u16),
      d: Unit,
      e: -2i32,
      f: true,
      g: 127u8,
    })
    .done();

  assert_eq!(have, want);

  let foo = RecordView::load(&have).unwrap();

  // TODO: fix
  // assert_eq!(foo.a(), 513);
  // assert_eq!(foo.b(), 1027);
  // assert_eq!(foo.c.zero(), 1541);
  // assert_eq!(foo.c.one(), 2055);
  // assert_eq!(foo.c(), Tuple(1541, 2055));
  // assert_eq!(foo.d(), Unit);
  // assert_eq!(foo.e(), -2);
  // assert_eq!(foo.f(), true);
  // assert_eq!(foo.f, true);
  // assert_eq!(foo.g(), 127);
  // assert_eq!(foo.g, 127);
}
