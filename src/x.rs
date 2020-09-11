// normal rust types (u16, etc)
// packed (don't exit in type system, just a wire format)
// offset exist in type system and wire format

// #[derive(X)]
struct Foo {
  a: u8,
}

trait X {
  type Builder;
}

impl X for Foo {
  type Builder = FooBuilder;
}

struct FooBuilderA<P> {
  allocator: (),
  allocation: &mut [u8; 1],
  parent: P,
}

impl FooBuilderA<P> {
  fn set(value: u8) -> P {}
}
