# rust-integeriser
Data-structures that represent values by unique integers.

## License

BSD-3-clause

## Documentation
* see `src/lib.rs`
* generate HTML-documentation with `cargo doc`

## Use
* Include the crate in your rust project by adding
  ```
  integeriser = { git = "https://github.com/tud-fop/rust-integeriser.git" }
  ```
  to the `[dependencies]` in your `Cargo.toml`.
* The crate contains a trait `integeriser::Integeriser` and two implementations of this trait `integeriser::{BTreeIntegeriser, HashIntegeriser}`.
