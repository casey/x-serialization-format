use core::{marker::PhantomData, mem::size_of};

/// The X trait maps native rust types to a `View` type, and a `Builder` type.
///
/// The View type is a repr(C), alignment(1) type which is the serialized
/// version of the type implementing the X trait.
///
/// The Builder type is a type that can serialize an X using an allocator.
/// The `P` type parameter is the outer object being serialized that contains
/// the value that the builder is serializing.
trait X {
  type View;
  type Builder<A: Allocator, P>: Builder<A>;
}

/// The only functionality exposed by the builder trait is the ability to
/// construct a new builder given an allocator.
trait Builder<A: Allocator> {
  fn new(allocator: A) -> Self;
}

/// The only functionality exposed by the Parent trait is the ability to
/// construct a new parent given the allocator.
///
/// I think, but am not certain, that although the Builder and Parent traits are
/// very similar, they both have to exist. Without the `Parent` trait, I was
/// unable to write the return type of `FooA::a`.
trait Parent<A: Allocator> {
  fn new(allocator: A) -> Self;
}

/// The Done struct isn't strictly necessary, but is used to add a final
/// `.done()` when serializing a message. Eventually, `done()` might return a
/// reference to the serialized root object, so you don't need to deserialize an
/// object you just serialized in order to access it.
#[must_use]
struct Done;

impl Done {
  fn done(self) {}
}

/// Done must implement Parent, so it can be fed into a chain of buidlers.
impl<A: Allocator> Parent<A> for Done {
  fn new(allocator: A) -> Self {
    Done
  }
}

/// An Allocator writes data being serialized to an underlying buffer.
///
/// `push`: Indicates that a new object of `length` bytes should be allocated.
///
/// `pop`: Indicates that we are done with the current object, and that the
/// allocator should return to writing to the last `push`ed object.
trait Allocator {
  fn push(&mut self, length: usize);

  fn pop(&mut self);

  fn write(&mut self, bytes: &[u8]);
}

/// An bump allocator with a fixed-size allocation. `push` and `pop` are just
/// stubs, and `write` doesn't do any error checking.
struct FixedAllocator<'a> {
  buffer: &'a mut [u8],
  offset: usize,
}

impl<'a> Allocator for FixedAllocator<'a> {
  fn push(&mut self, size: usize) {}

  fn pop(&mut self) {}

  fn write(&mut self, bytes: &[u8]) {
    for (dst, src) in self.buffer[self.offset..].iter_mut().zip(bytes) {
      *dst = *src;
    }
    self.offset += bytes.len();
  }
}

/// An X implementation for a simple type, u16.
impl X for u16 {
  type Builder<A: Allocator, P> = U16Builder<A, P>;
  type View = U16;
}

/// U16::View holds the bytes of little-endian u16.
#[repr(C)]
struct U16 {
  bytes: [u8; 2],
}

impl From<U16> for u16 {
  fn from(value: U16) -> u16 {
    u16::from_le_bytes(value.bytes)
  }
}

/// A U16 builder that contains an allocator and a phantom parent.
struct U16Builder<A: Allocator, P>(A, PhantomData<P>);

/// The only method the builder supports is `set()`, which writes a u16 to the
/// allocator, and constructs and returns the parent constructed with its
/// allocator.
impl<A: Allocator, P: Parent<A>> U16Builder<A, P> {
  fn set(mut self, value: u16) -> P {
    self.0.write(&value.to_le_bytes());
    P::new(self.0)
  }
}

/// Builder implementation for U16 builder, which constructs a new builder given
/// an allocator.
impl<A: Allocator, P> Builder<A> for U16Builder<A, P> {
  fn new(allocator: A) -> Self {
    U16Builder(allocator, PhantomData)
  }
}

/// For a while I thought that the best way for users to use the macros would be
/// to use `View` types directly, and create structs that are all composed of
/// `View` types.
///
/// Now, I think the best interface is actually for users to always write native
/// rust types, and then derive the `View` types from those native rust
/// definition.
///
/// In the following code, the user writes a `Foo` type with native rust types,
/// and a `FooStruct` type composed of the corresponding view types is generated
/// by the macros.
///
/// Advantages:
/// - Users don't need to learn all the view type equivalents of the native rust
///   types that they're already familiar with.
///
/// - Users don't need to learn how to write `impl` blocks with empty methods,
///   which is the way I was considering having users write tables.
///
/// - The native rust types can optionally be used as arguments to the generated
///   builders, as is done with `FooA::set`.
///
/// - A native rust equivalent always exists to the flat types. For example, you
///   might wish to deserialize a message, and then pass values from that
///   message to other parts of your codebase. If you don't mind paying for the
///   conversion, you can always convert those sub-values to native rust types,
///   so that other parts of the codebase don't need to know about the weird X
///   types.
///
/// - I think it's more obvious what the generated type is going to be.
///
/// Disadantages:
///
/// - Sometimes users will never use the native types that they write.

/// Here's an example derive invocation:

// #[derive(X)]
// #[x(struct = FooStruct)]
struct Foo {
  a: u16,
  b: u16,
}

impl Foo {
  fn store<A: Allocator>(allocator: A) -> <Foo as X>::Builder<A, Done> {
    <Foo as X>::Builder::<A, Done>::new(allocator)
  }
}

/// Which will generate this X impl:

impl X for Foo {
  type Builder<A: Allocator, P> = FooA<A, P>;
  type View = FooStruct;
}

/// Which will generate this View type for Foo:

#[repr(C)]
struct FooStruct {
  a: <u16 as X>::View,
  b: <u16 as X>::View,
}

/// Generated conversion from view to native:

impl From<FooStruct> for Foo {
  fn from(value: FooStruct) -> Self {
    Self {
      a: value.a.into(),
      b: value.b.into(),
    }
  }
}

/// Foo's builder. The first in a chain of builders, one for each field:
struct FooA<A: Allocator, P>(A, PhantomData<P>);

/// Generated builder implementation for FooA, foo's builder:
///
/// (This is not used in this example, but would be if foo was a sub-object in a
/// larger composite type.)
impl<A: Allocator, P> Builder<A> for FooA<A, P> {
  fn new(allocator: A) -> Self {
    FooA(allocator, PhantomData)
  }
}

impl<A: Allocator, P: Parent<A>> FooA<A, P> {
  /// Figuring out what traits I needed to create in order to write this
  /// signature was the hardest part of this whole ordeal.
  fn a(self) -> <u16 as X>::Builder<A, FooB<A, P>> {
    <u16 as X>::Builder::new(self.0)
  }

  fn set(self, foo: Foo) -> P {
    self.a().set(foo.a).b().set(foo.b)
  }
}

/// The second and last of Foo's builders:

struct FooB<A: Allocator, P>(A, PhantomData<P>);

impl<A: Allocator, P: Parent<A>> FooB<A, P> {
  fn b(self) -> <u16 as X>::Builder<A, P> {
    <u16 as X>::Builder::new(self.0)
  }
}

/// An implementation of Parent for FooB, so the sub-object in `Foo::a` can
/// return to foo and begin serializing `Foo::b`
impl<A: Allocator, P> Parent<A> for FooB<A, P> {
  fn new(allocator: A) -> Self {
    FooB(allocator, PhantomData)
  }
}

#[test]
fn construct() {
  let buffer = &mut [0, 0, 0, 0];

  let allocator = FixedAllocator { buffer, offset: 0 };

  /// THE BIG PAY OFF. CHECK OUT HOW FUCKING CLEAN IT IS. ANY DEVIATION FROM
  /// CANONICAL ORDERING IS SWIFTLY PUNISHED WITH BRUTAL TYPE ERRORS.
  Foo::store(allocator).a().set(513).b().set(1027).done();

  assert_eq!(buffer, &[1, 2, 3, 4]);
}
