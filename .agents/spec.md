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
* `impl IntoIterator<Item = I>`
* `impl futures::Stream<Item = I>`

Notes:

* Some payloads have generic types that implement specific traits
  * Examples
    * `impl futures::Stream<Item = I>`

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

### Moniker

Moniker is a string used in [conversion trait](#conversion-trait) name.

If the item comes from a [foundational crate](#foundational-crate), then the item moniker is just the name of the item, else it's a concatentation of the crate name in PascalCase and an item name.

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

* Some round-trip pairs of conversion traits
* Some conversion traits are sync
  * Examples:
    * `ValueToFile`
* Some conversion traits are async (if the payload or storage is inherently `async`)
  * Examples:
    * `ValueToTokioFile`
    * `StreamToFile`
* Conversion from `Vec<T>` is covered by `Iterator*` family of conversion traits, which accept `iter: I` where `I: IntoIterator`
* Converson from `Box<T>` is covered by `Value*` family of conversion traits, which accept `value: V` where `V: Borrow<T>`

### Mirror pair

Mirror pair is a pair of [conversion traits](#conversion-trait) where the first trait implements serialization and the second trait implements deserialization between the same [payload](#payload) and [storage](#storage).

Notes:

* Mirror pairs must pass the round-trip test
* Mirror pairs may have different `Error` types

### Foundational crate

Foundational crate is a crate that is (subjectively) very popular in the ecosystem.

Values (exhaustive):

* `std`
* `futures`
