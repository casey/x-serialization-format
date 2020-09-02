# data

This library implements a serialization format for Rust. The name "data" is a
placeholder, hopefully someone comes up with a better one soon.

This project is currently dreamware. It exists only as documentation, issues,
and figments of the imagination.

That being said, this readme is written as if everything is already done, so I
don't have to go and rewrite it later.

## Features

- Fast: Serialization and deserialization are efficient, making Data a good
  format for use in resource-constrained environments.

- Excellent Rust integration: Standard library types are supported where
  possible, and enum access is idiomatic and ergonomic.

- No external schema language or additional build steps required: Schemas are
  declared in Rust with procedural macros.

- Schema evolution: Fields can be added and optional fields removed without
  breaking backwards compatibility.

- `nostd` support: Neither the standard library nor the `alloc` crate are
  required to serialize or deserialize messages.

- Zero parse: Accessing fields of a message only requires loading and following
  offsets, making it very fast. There is, however, an up-front validation step,
  albeit a very efficient one.

- Zero copy: Deserialization and access do not require memory beyond that
  used to store the message itself.

- `mmap`-friendly: Messages can be mapped into memory and accessed in-place, making
  Data a good choice for large messages and technologies like LMDB.

- Canonicality: For any message, a canonical serialization of that message is
  defined. For ergonomic and efficiency reasons, this may be opt-in.

- Simple: The encoding is straightforward and easy to understand. If you
  absolutely had to, writing your own deserializer or serializer would be
  doable.

## Non-goals

- Languages other than Rust: Currency, only Rust support is planned. If there
  is demand, support for other languages may implemented, but it isn't a
  near-term goal.

## Prior Art

Data draws inspiration from many other formats.

The most similar format is probably
[FIDL](https://fuchsia.dev/fuchsia-src/development/languages/fidl), the Fuchsia
Interface Definition Language, a schema language and wire format used for
inter-process communication in the Fuchsia operating system.

Unfortunately, FIDL is difficult to use outside of the Fuchsia source tree,
requires an external schema, and use-cases outside of Fuchsia are not
well-supported.

Data is also very similar to Flatbuffers and Cap'n Proto, but differs in that
it does not require an external schema definition, and is designed from the
ground up to have excellent Rust support.

## Encoding

Message encoding is hopefully straightforward, and is intended to be both
simple and preferment.

Values have no alignment guarantees, so sequential elements are always directly
adjacent, with no padding. In general, modern computers have little to no
unaligned access penalty, so the format avoids the complexity of guaranteeing
alignment, and the space overhead of padding.

Integers are little-endian, regardless of platform, since most computers are
little-endian.

In order to support efficient traversal, all variable-length data is stored
out-of-line as a relative offset. Since the length of non-variable length data
is known in advance, traversal never requires inspecting so as to be able to
skip over variable length data.

By way of illustration, consider the following struct:

```rust
use data::{Slice, U8, U16, U32};

#[derive(Data)]
#[repr(C)]
struct Foo {
  a: U8,
  b: Slice<U16>,
  c: U32,
}
```

Conceptually, this struct contains a `u8`, followed by a variable number of
`u16`s, and finally followed by a `u32`.

When serialized, the struct will have the following layout in memory:

```
0x0000 A         # 1 byte value of 'a'
0x0001 OOOO OOOO # 8 byte offset to 'b'
0x0009 LLLL LLLL # 8 byte length of 'b'
0x0011 CCCC      # 4 byte value of 'c'
0x0015 BBBBBBBB… # contents of 'b'
```

Notice that because 'b' is stored out-of-line, we can access 'c' directly.
Given a pointer `p` to `Foo`, `c` will always be at `p + 0x11`.

To access variable length data, in this case 'b', we calculate the pointer to
the offset, `p + 0x1`, load the value it points to, and add it to the pointer
to the offset.

In pseudocode:

```rust
fn load_slice(pointer: *const u64) -> &[u8] {
  let offset = *pointer;
  let len = *(pointer + 0x8);
  let data = pointer + offset;
  std::slice::from_raw_parts(data, len)
}
```

Offsets are relative to their own location, so zero is not a valid offset,
since that would point to the offset itself. So, zero is used as a sentinel
value, for example to represent `Option::None`.

### Types

#### `()`

Since `()`, also called the unit type, has no values, it requires zero bits to
represent, and thus the encoding is the empty byte string.

#### `bool`

Boolean `true` is encoded as a byte containing the value `1`, and `false` is
encoded as a byte containing the value `0`.

#### `u8`, `u16`, `u32`, `u64`, `u128`

Unsigned integers are encoded as little endian, and are fixed width. `u8` is
always 1 byte, `u16` two bytes, and so on.

#### `i8`, `i16`, `i32`, `i64`, `i128`

Signed integers are encoded as little endian two's complement, and are fixed
width. `i8` is always 1 byte, `i16` two bytes, and so on.

#### `usize`, `isize`

`usize` and `isize` are always encoded as a `u64` and a `i64`, respectively,
regardless of platform. However, validation will fail if the encoded value is
too big to fit into the platform's size type.

#### `char`

Characters are encoded as a `u32` containing the Unicode scalar value they
represent.

#### `&str`

Strings are encoded as a relative offset pointing to the UTF-8 encoded contents
of the string, and the length of the contents in bytes, encoded as a `u64`

#### `&[T]`

Slices are encoded as a relative offset pointing to the contents of the slice,
and the length of the slice. The length is the number of elements in the slice,
not the number of bytes.

#### `Option<T>`

Options that contain a value are encoded as a relative offset to the contained
`T`. `None` is encoded as a zero offset.

#### `Result<T, E>`

Results are encoded as a byte containing `0` for `Ok` or `1` for `Err`,
followed in either case by a relative offset to the contained `T` or `E`.

#### Structs

Structs are encoded as their fields, in-order. Because structs are not encoded
with any indication of how many fields they contain, it is not possible to make
backwards compatible changes to structs. For compound types which might need to
change in the future, see tables.

#### Enums

The different values of an enum are called its “variants”. These are identified
by an unsigned integer called the enum's “discriminant”.

Enums are encoded by first encoded the discriminant as a `u64`, followed by the
fields of the enum.

Because the encoding tells the decoder the enum's variant, variants can be
added and removed from enums.

### Tables

Tables are like structs, but fields can be added and removed without breaking
backwards compatibility.

Tables are encoded as a `u64` `len`, followed by `len` count relative offsets
to the fields of the table.
