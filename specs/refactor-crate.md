# Refactor the whole crate

## Tasks

* Refactor the crate according to .agents/spec.md
* Reimplement `Format` as an envelope enum (one variant per format, holding the format unit struct)
* Remove the following methods from `Format`:
  * `print_one`
  * `eprint_one`
  * `println_one`
  * `eprintln_one`
  * `write_one`
  * `writeln_one`
