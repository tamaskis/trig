//! [![github]](https://github.com/tamaskis/trig)&ensp;[![crates-io]](https://crates.io/crates/trig)&ensp;[![docs-rs]](https://docs.rs/trig)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! Complete set of trigonometric and hyperbolic functions in Rust.
//!
//! # Summary
//!
//! This crate defines the [`Trig`] trait defining the complete set of trigonometric and hyperbolic
//! functions. Rust already provides the following functions for [`f32`]s and [`f64`]s:
//!
//! * trigonometric functions (radians): `sin`, `cos`, `tan`
//! * inverse trigonometric functions (radians): `asin`, `acos`, `atan`, `atan2`
//! * hyperbolic functions: `sinh`, `cosh`, `tanh`
//! * inverse hyperbolic functions: `asinh`, `acosh`, `atanh`
//!
//! In addition to these functions, the [`Trig`] trait also defines the reciprocal functions and
//! their inverses for both the trigonometric and hyperbolic functions. The complete set of methods
//! defined on the [`Trig`] trait is:
//!
//! * trigonometric functions (radians): `sin`, `cos`, `tan`, `csc`, `sec`, `cot`
//! * inverse trigonometric functions (radians): `asin`, `acos`, `atan`, `acsc`, `asec`, `acot`
//! * trigonometric functions (degrees): `sind`, `cosd`, `tand`, `cscd`, `secd`, `cotd`
//! * inverse trigonometric functions (degrees): `asind`, `acosd`, `atand`, `atan2d`, `acscd`,
//!   `asecd`, `acotd`
//! * hyperbolic functions: `sinh`, `cosh`, `tanh`, `csch`, `sech`, `coth`
//! * inverse hyperbolic functions: `asinh`, `acosh`, `atanh`, `acsch`, `asech`, `acoth`
//! * unit conversions: `deg2rad`, `rad2deg`
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

// Module declarations.
pub(crate) mod f32_impl;
pub(crate) mod f64_impl;
pub(crate) mod trig_trait;

// Re-exports.
pub use crate::trig_trait::Trig;
