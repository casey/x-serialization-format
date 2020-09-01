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

- `nostd` support: Neither the standard library nor the alloc crate are
  required to serialize or deserialize messages.

- Zero parse: Accessing fields of a message only requires loading and following
  offsets, making it very fast. There is, however, an up-front validation step,
  albeit a very efficient one.

- Zero copy: Deserialization and access do not require memory beyond that
  used to store the message itself.

- Mmap-friendly: Messages can be mapped into memory and accessed in-place, making
  Data a good choice for large messages and technologies like LMDB.

- Canonicality: For any message, a canonical serialization of that message is
  defined. For ergonomic and efficiency reasons, this may be opt-in.

- Simple: The encoding is straightforward and easy to understand. If you
  absolutely had to, writing your own deserializer or serializer would be
  doable.

## Non-goals

- Languages other than Rust: Currenly, only Rust support is planned. If there
  is demand, suppport for other languages may implemented, but it isn't a
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
