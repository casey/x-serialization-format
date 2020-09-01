# data

This library implements a serialization format for Rust. The name "data" is a
placeholder, hopefully someone comes up with a better one soon.

The current status of this project can best be described as dreamware. It
exists only as documentation, issues, and speculation. That being said, this
readme is written as if the software already exists, so I don't have to go and
rewrite it later.

## Features

- Fast: Serialization and deserialization are extremely efficient, and a good
  choice for resource-constrained environments.

- Excellent Rust integration: All relevant types from the standard library are
  supported.

- No external schema language or additional build step: Objects and their
  schemas are declared in Rust with procedural macros.

- Schema evolution: Adding or removing fields from tables is supported.

- `nostd` support: Neither the standard library nor the alloc crate are
  required to serialize or deserialize messages.

- Zero parse: Accessing fields of a message only requires loading and following
  offsets, making it very efficient. There is, however, an up-front validation
  step, albeit a very efficient one.

- Zero copy: Deserialization and access do not require memory beyond that
  used to store the message itself.

- Mmap-friendly: Messages can be mapped into memory and accessed in-place, making
  data a good choice for large messages and technologies like LMDB.

- Canonicality: For any message, a canonical serialization of that message is
  defined. For ergonomic and efficiency reasons, this may be opt-in.

- Simple: Data encoding is straightforward and easy to understand. If you
  absolutely had to, writing your own deserializer or serializer should be
  doable.

## Non-goals

- Languages other than Rust: Currenly, only Rust support is planned. If there
  is demand, suppport for other languages may implemented, but it isn't a
  near-term goal.

## Prior Art

Data is draws inspiration from a ton of other formats.

The format most similar to Data is probably
[FIDL](https://fuchsia.dev/fuchsia-src/development/languages/fidl), the Fuchsia
Interface Definition Language, a schema language and wire format used for
inter-process communication in the Fuchsia operating system.

Unfortunately, FIDL is difficult to use outside of the Fuchsia source tree, and
use-cases outside of Fuchsia are not well-supported.

Data is also very similar to Flatbuffers and Cap'n Proto, but differs in that
it does not require an external schema definition, and is designed from the
ground up to support Rust idioms.
