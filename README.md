# `gp2y0e02b`

> no_std driver for [GP2Y0E02B](https://www.sharpsde.com/fileadmin/products/Optoelectronics/Sensors/Specs/GP2Y0E02B_TI_OP13004EN.pdf) (SHARP I2C Distance Measuring Sensor, 4-50cm)

[![Build Status](https://github.com/lucazulian/gp2y0e02b/workflows/gp2y0e02b-ci/badge.svg)](https://github.com/lucazulian/gp2y0e02b/actions?query=workflow%3Agp2y0e02b-ci)
[![crates.io](https://img.shields.io/crates/v/gp2y0e02b.svg)](https://crates.io/crates/gp2y0e02b)
[![Docs](https://docs.rs/gp2y0e02b/badge.svg)](https://docs.rs/gp2y0e02b)

## Basic usage

Include this [library](https://crates.io/crates/gp2y0e02b) as a dependency in your `Cargo.toml`:

```yaml
[dependencies.gp2y0e02b]
version = "<version>"
```

Use [embedded-hal](https://github.com/rust-embedded/embedded-hal) implementation to get I2C handle and then create gp2y0e02b handle.

```rust
extern crate gp2y0e02b;
```

## License

[MIT license](http://opensource.org/licenses/MIT)
