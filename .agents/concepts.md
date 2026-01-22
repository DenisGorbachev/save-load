# Concepts

## `save-load` package

A Rust package that provides envelope enums that implement functionality common for multiple file formats.

* MSRV: 1.75
  * Needs to support `async fn` in traits

## Definitions

### Payload

An actual data value.

Examples:

* `T`
* `&T`
* `[T; N]`
* `&[T]`
* `Vec<T>`
* `VecDeque<T>`
* `Box<T>`
* `Box<[T]>`
* `I: IntoIterator<Item = P>` where `P: Borrow<T>`
* `S: futures::Stream<Item = P>` where `P: Borrow<T>`

Notes:

* Some payloads have generic types that implement specific traits
  * Examples
    * `S: futures::Stream<Item = P>` where `P: Borrow<T>`

### Storage

An IO source/sink or location for serialized bytes of the actual data value.

Examples:

* `std::fs::File`
* `tokio::fs::File`
* `std::path::PathBuf`
* `impl Read`
* `impl BufRead`
* `impl Write`
* `std::io::BufWriter<W>`
* `impl futures::io::AsyncRead`
* `impl futures::io::AsyncWrite`
* `impl tokio::io::AsyncRead`
* `impl tokio::io::AsyncWrite`

Notes:

* Some storages are read-only
  * Examples
    * `impl Read`
* Some storages are write-only
  * Examples
    * `impl Write`
* `futures::io::AsyncRead` and `tokio::io::AsyncRead` have different signatures
* `futures::io::AsyncWrite` and `tokio::io::AsyncWrite` have different signatures

### Format

A set of rules for serializing and deserializing the [payload](#payload) in [storage](#storage).

Examples:

* `Json`
* `Jsonl`
* `Toml`
* `Csv`
* `Yaml`
* `Xml`

Requirements:

* Must be implemented as a unit struct with no fields
  * Note that configuration of the serialization/deserialization is explicitly not supported (must use sane defaults)
* Must be feature-gated
  * The `cfg` attribute must be placed on the `mod` and `pub use` declarations (not on individual items)
  * The corresponding dependency must be `optional`

Preferences:

* Should implement as many conversion traits as possible

### Inner type

The type being serialized / deserialized in the [payload](#payload).

Notes:

* In the [payload definition](#payload), inner type is called `T`.

### Outer item

Outer item is the outermost item in the [payload](#payload).

Notes:

* Outer item may be a specific type or a generic type that implements a specific trait (e.g. a `T` or `I: IntoIterator<Item = P>` where `P: Borrow<T>`)

### Moniker

An identifier that is a part of [conversion trait](#conversion-trait) name.

Examples:

* `std::fs::File` -> `File`
* `std::iter::Iterator` -> `Iterator`
* `tokio::fs::File` -> `TokioFile`

Notes:

* How to generate the monikers for a list of items:
  * Order the items by popularity descending (defined as count of dependents on crates.io on 2026-01-01)
  * For each item:
    * Calculate the minimum possible moniker that is still available (not taken by another item):
      * `{ident}` (primary identifier of the item) (e.g. `File`)
      * `{crate}{ident}` (the crate name in PascalCase + primary identifier of the item) (e.g. `TokioFile`)
      * `{path}{ident}` (the full path including the crate name in PascalCase + primary identifier of the item) (e.g. `TokioFsFile`)

### Conversion trait

A trait with a single fn that implements serialization or deserialization between a specific [payload](#payload) and a specific [storage](#storage).

Examples:

* `ValueToFile`
* `FileToValue`
* `ValueToTokioFile`
* `TokioFileToValue`
* `IteratorToFile`
* `StreamToFile`
* `StreamToPathBuf`
* `ValueToPathBuf`

Requirements:

* Name must contain a [payload](#payload) [moniker](#moniker)
* Name must contain a [storage](#storage) [moniker](#moniker)
* Associated types must have corresponding trait bounds
  * Examples
    * In `FileToIter` trait, the `Output` must have `Iterator` trait bound
* Method should have generic parameters instead of `impl` parameters
* Method generic parameters must have `T` as the first parameter
* Method must return a `Result<Self::Output, Self::Error>`
  * `Output` and `Error` must be associated types of the trait
* Implementation code for error handling must use the handle-family macros from `errgonomic` crate (example: `use errgonomic::handle;` + `handle!(...)`)

Preferences:

* Should accept both references and owned values
  * Examples:
    * `V: Borrow<T>`
    * `I: IntoIterator<Item = V>` where `V: Borrow<T>`
* Associated types should be specific types instead of trait objects (prefer specific types instead of `Box<dyn ...>` if possible)
* Methods that write to storage should return every owned value that is created within the method
  * Examples
    * If the method creates a `File` for writing the payload, it should return it

Notes:

* Some conversion traits are sync
  * Examples:
    * `ValueToFile`
* Some conversion traits are async (if the payload or storage is inherently `async`)
  * Examples:
    * `ValueToTokioFile`
    * `StreamToFile`
* Conversion from `Vec<T>` is covered by `Iterator*` family of conversion traits, which accept `iter: I` where `I: IntoIterator`
* Conversion from `Box<T>` is covered by `Value*` family of conversion traits, which accept `value: V` where `V: Borrow<T>`
* Async conversion methods must return `impl Future` with explicit bounds

### Conversion trait impl

An implementation of a [conversion trait](#conversion-trait).

### Mirror pair

A pair of [conversion traits](#conversion-trait) where the first trait implements serialization and the second trait implements deserialization between the same [payload](#payload) and [storage](#storage).

Examples:

* `FileToIter` and `IterToFile`

Notes:

* Mirror pairs must pass the serialize-deserialize round-trip test
* Mirror pairs may not pass a deserialize-serialize round-trip test for formats that accept the map keys in arbitrary order, and the key order information is lost during deserialization
* Mirror pairs may have different `Error` types

### Embedding pair

A pair of [conversion traits](#conversion-trait) where the both traits have the same inputs but different outputs, and the output of the first trait can be losslessly converted into the output of the second trait (i.e. there is an injection between output of first trait and the output of second trait).

Examples:

* `FileToIter` and `FileToIterOfOptions` (note that `T` can be losslessly converted into `Option<T>`)
* `FileToIter` and `FileToIterOfResults` (note that `T` can be losslessly converted into `Result<T, E>`)
* `FileToVec` and `FileToIter` (note that `Vec<T>` can be losslessly converted into `I: Iterator<Item = T>`)
