# Specification for this crate

## Definitions

### Payload

Payload is an actual data value.

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

Storage is a locator of the actual data value.

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

* Examples
  * `Json`
  * `Jsonl`
  * `Toml`
  * `Csv`

Requirements:

* Must be implemented as a unit struct with no fields

Preferences:

* Should implement as many conversion traits as possible

### Inner type

Inner type is the type being serialized / deserialized in the [payload](#payload).

In the [payload definition](#payload), inner type is called `T`.

### Outer item

Outer item is the outermost item in the [payload](#payload).

Notes:

* Outer item may be a specific type or a generic type that implements a specific trait (e.g. a `T` or `I: IntoIterator<Item = P>` where `P: Borrow<T>`)

### Moniker

Moniker is an identifier that is a part of [conversion trait](#conversion-trait) name.

How to generate the monikers for a list of items:

* Order the items by popularity descending (defined as count of dependents on crates.io)
* For each item:
  * Calculate the minimum possible moniker that is still available (not taken by another item):
    * `{ident}` (primary identifier of the item) (e.g. `File`)
    * `{crate}{ident}` (the crate name in PascalCase + primary identifier of the item) (e.g. `TokioFile`)
    * `{path}{ident}` (the full path including the crate name in PascalCase + primary identifier of the item) (e.g. `TokioFsFile`)

Examples:

* `std::fs::File` -> `File`
* `std::iter::Iterator` -> `Iterator`
* `tokio::fs::File` -> `TokioFile`

### Conversion trait

Conversion trait is a trait with functions that implement serialization and deserialization between a specific [payload](#payload) and a specific [storage](#storage).

Examples:

* `ValueToFile`
* `FileToValue`
* `ValueToTokioFile`
* `TokioFileToValue`
* `IteratorToFile`
* `StreamToFile`
* `StreamToPathBuf`

Requirements:

* Name must contain a [payload](#payload) [moniker](#moniker)
* Name must contain a [storage](#storage) [moniker](#moniker)
* Method should have generic parameters instead of `impl` parameters
* Method generic parameters must have `T` as the first parameter
* Method must return a `Result<Self::Output, Self::Error>`
  * `Output` and `Error` must be associated types of the trait

Preferences:

* Should accept both references and owned values
  * Examples:
    * `V: Borrow<T>`
    * `I: IntoIterator<Item = V>` where `V: Borrow<T>`
* Should return every owned value that is created within the method
  * Examples
    * Should return the `File`

Notes:

* Some conversion traits are sync
  * Examples:
    * `ValueToFile`
* Some conversion traits are async (if the payload or storage is inherently `async`)
  * Examples:
    * `ValueToTokioFile`
    * `StreamToFile`
* Conversion from `Vec<T>` is covered by `Iterator*` family of conversion traits, which accept `iter: I` where `I: IntoIterator`
* Converson from `Box<T>` is covered by `Value*` family of conversion traits, which accept `value: V` where `V: Borrow<T>`
* Async conversion traits must be implemented with `async fn` (not `#[async_trait]`)

### Mirror pair

Mirror pair is a pair of [conversion traits](#conversion-trait) where the first trait implements serialization and the second trait implements deserialization between the same [payload](#payload) and [storage](#storage).

Notes:

* Mirror pairs must pass the serialize-deserialize round-trip test
* Mirror pairs may not pass a deserialize-serialize round-trip test for formats that accept the map keys in arbitrary order, and the key order information is lost during deserialization
* Mirror pairs may have different `Error` types
