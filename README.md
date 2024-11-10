# trig

[<img alt="github" src="https://img.shields.io/badge/github-tamaskis/trig-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/tamaskis/trig)
[<img alt="crates.io" src="https://img.shields.io/crates/v/trig.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/trig)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-trig-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/trig)

Complete set of trigonometric functions in Rust.

## Documentation

Please see https://docs.rs/trig.

## Examples

```rust
use trig::Trig;

let x = std::f64::consts::FRAC_PI_2;
let abs_difference = (x.csc() - 1.0).abs();

assert!(abs_difference < 1e-16);
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or 
<a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
</sub>