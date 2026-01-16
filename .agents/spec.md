# Specification for this crate

* Every format must be implemented as a unit struct with no fields
  * Examples
    * `Json`
    * `Jsonl`
    * `Toml`
    * `Csv`
* Every format must implement all conversion traits

## Definitions

### Payload

Payload is a value that represents the data in volatile memory (e.g. RAM).

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
* `impl Stream<Item = I>`

Notes:

* Some payloads have generic types that implement specific traits
  * Examples
    * `impl Stream<Item = I>`

### Storage

Storage is a value that represents the data in persistent memory (e.g. on disk).

Examples:

* `std::fs::File`
* `tokio::fs::File`
* `std::path::PathBuf`
* `impl Read`
* `impl BufRead`
* `impl Write`
* `impl BufWrite`
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

### Moniker

Moniker is a string used in [conversion trait](#conversion-trait) name.

If the item comes from a [foundational crate](#foundational-crate), then the item moniker is just the name of the item, else it's a concatentation of the crate name and an item name.

Examples:

* `std::fs::File` -> `File`
* `std::iter::Iterator` -> `File`
* `tokio::fs::File` -> `TokioFile`

### Conversion trait

Conversion trait is a trait with functions that implement serialization and deserialization between a specific [payload](#payload) and a specific [storage](#storage).

Examples:

* `ValueToFile`
* `FileToValue`
* `ValueToTokioFile`
* `TokioFileToValue`
* `IterToFile`
* `StreamToFile`
* `StreamToPathBuf`

Requirements:

* Name must contain a [value](#payload) [moniker](#moniker)
* Name must contain a [storage](#storage) [moniker](#moniker)
* Method should have generic parameters instead of `impl` parameters
* Method generic parameters must have `T` as the first parameter
* Method must return a `Result<Self::Output, Self::Error>`
  * `Output` and `Error` must be associated types of the trait

Preferences:

* Should accept an `impl Into*` instead of plain `&T` or `T`
  * Examples
    * `impl Into<P>` where `P: Borrow<T>`
    * `impl IntoIterator<P>` where `P: Borrow<T>`
* Should accept an `T, P: Borrow<T>` instead of plain `T` (to accept both references and owned values)
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
* Conversion from `Vec<T>` is covered by `Iter*` family of conversion traits, which accept `payload: impl IntoIterator`
* Converson from `Box<T>` is covered by `Value*` family of conversion traits, which accept `payload: P` where `P: Borrow<T>`

### Mirror pair

Mirror pair is a pair of [conversion traits](#conversion-trait) where the first trait implements serialization and the second trait implements deserialization between the same [payload](#payload) and [storage](#storage).

Notes:

* Mirror pairs must pass the round-trip test
* Mirror pairs may have different `Error` types

### Foundational crate

Foundational crate is a crate that is (subjectively) very popular in the ecosystem.

Examples:

* `std`
* `futures`
