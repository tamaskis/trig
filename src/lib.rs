//! [![github]](https://github.com/tamaskis/trig)&ensp;[![crates-io]](https://crates.io/crates/trig)&ensp;[![docs-rs]](https://docs.rs/trig)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Complete set of trigonometric functions in Rust.
//!
//! # Summary
//!
//! This crate defines the [`Trig`] trait defining the complete set of trigonometric functions. Rust already provides the following trigonometric functions for [`f32`]s and [`f64`]s:
//!
//! * `sin`
//! * `cos`
//! * `tan`
//! * `asin`
//! * `acos`
//! * `atan`
//! * `atan2`
//!
//! The [`Trig`] trait also includes these functions, but also defines the following additional trigonometric functions:
//!
//! * `csc`
//! * `sec`
//! * `cot`
//! * `acsc`
//! * `asec`
//! * `acot`
//! * `sind`
//! * `cosd`
//! * `tand`
//! * `cscd`
//! * `secd`
//! * `cotd`
//! * `asind`
//! * `acosd`
//! * `atand`
//! * `acscd`
//! * `asecd`
//! * `acotd`
//!
//! Additionally, the following unit conversion methods are also defined:
//!
//! * `deg2rad`
//! * `rad2deg`
//!
//! # Implementations
//!
//! This crate currently implements the [`Trig`] trait for the following types/structs:
//!
//! * [`f32`]
//! * [`f64`]
//!
//! # Example
//!
//! ```
//! use trig::Trig;
//!
//! let x = std::f64::consts::FRAC_PI_2;
//! let abs_difference = (x.csc() - 1.0).abs();
//!
//! assert!(abs_difference < 1e-16);
//! ```

// Linter setup.
#![warn(missing_docs)]

// Linking project modules.
pub(crate) mod f32_impl;
pub(crate) mod f64_impl;
pub(crate) mod trig_trait;

// Re-exports.
pub use crate::trig_trait::Trig;
