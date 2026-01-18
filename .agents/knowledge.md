# Knowledge

## General

* Some formats do support deserializing a file in chunks
  * Examples
    * JSONL
    * CSV
* Some formats don't support deserializing a file in chunks
  * Examples
    * JSON
    * XML
    * TOML
* Some formats do have clear delimiters for array values
  * Examples
    * XML: closing tag
    * JSON: comma
    * CSV: newline
* Some formats don't have clear delimiters for array values
  * Examples
    * TOML
* Some formats don't support arrays at the top level
  * Examples
    * TOML

## Dependencies

* `quick-xml` contains optional support of asynchronous reading and writing using tokio. To get it enable the `async-tokio` feature.
* Some format implementations return iterators of `Result<T, E>` instead of iterators of `T`
  * Examples
    * `csv`
