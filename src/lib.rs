//! [![github]](https://github.com/tamaskis/trig)&ensp;[![crates-io]](https://crates.io/crates/trig)&ensp;[![docs-rs]](https://docs.rs/trig)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Additional trigonometric functions in Rust.

// Linter setup.
#![warn(missing_docs)]

// Linking project modules.
pub(crate) mod module;

// Re-exports.
pub use crate::module::example_function;
