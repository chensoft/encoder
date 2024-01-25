Encoder
==========================

Blazing fast encoder written in Rust

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][license-badge]][license-url]
[![Documentation][document-badge]][document-url]
[![Build Status][linux-badge]][linux-url]
[![Build Status][macos-badge]][macos-url]
[![Build Status][windows-badge]][windows-url]

[crates-badge]: https://img.shields.io/crates/v/encoder.svg
[crates-url]: https://crates.io/crates/encoder
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-url]: https://github.com/chensoft/encoder/blob/master/LICENSE
[document-badge]: https://docs.rs/encoder/badge.svg
[document-url]: https://docs.rs/encoder
[linux-badge]: https://github.com/chensoft/encoder/actions/workflows/linux.yml/badge.svg
[linux-url]: https://github.com/chensoft/encoder/actions/workflows/linux.yml
[macos-badge]: https://github.com/chensoft/encoder/actions/workflows/macos.yml/badge.svg
[macos-url]: https://github.com/chensoft/encoder/actions/workflows/macos.yml
[windows-badge]: https://github.com/chensoft/encoder/actions/workflows/windows.yml/badge.svg
[windows-url]: https://github.com/chensoft/encoder/actions/workflows/windows.yml

## Quick Start

#### Number encoding

```rust
use encoder::number::Encode;

fn main() {
    let mut buf = vec![];
    1_i32.encode(&mut buf);
    2_u32.encode(&mut buf);
    1_f32.encode(&mut buf);
    2_f64.encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#"121.02.0"#);

    assert_eq!(1_i32.stringify(), "1");
    assert_eq!(2_u32.stringify(), "2");
    assert_eq!(1_f32.stringify(), "1.0");
    assert_eq!(2_f64.stringify(), "2.0");
}
```

#### Json encoding

```rust
use encoder::json::Encode;

fn main() {
    let mut buf = vec![];
    "Hello, 世界! 👋".encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#""Hello, 世界! 👋""#);

    assert_eq!("Hello, 世界! 👋".stringify(), r#""Hello, 世界! 👋""#);
}
```

## Benchmark

- MacBook Air, Apple M2 24G, macOS 14.2.1

| Name      |              Time               |
|:----------|:-------------------------------:|
| i8_max    | [1.1761 ns 1.1767 ns 1.1775 ns] |
| i8_min    | [1.1752 ns 1.1754 ns 1.1757 ns] |
| u8_max    | [1.1759 ns 1.1761 ns 1.1764 ns] |
| u8_min    | [1.1759 ns 1.1761 ns 1.1763 ns] |
| i16_max   | [8.0300 ns 8.0445 ns 8.0584 ns] |
| i16_min   | [8.1324 ns 8.1535 ns 8.1812 ns] |
| u16_max   | [1.4697 ns 1.4698 ns 1.4700 ns] |
| u16_min   | [1.1757 ns 1.1759 ns 1.1762 ns] |
| i32_max   | [7.0862 ns 7.0872 ns 7.0883 ns] |
| i32_min   | [8.5705 ns 8.6005 ns 8.6398 ns] |
| u32_max   | [1.3195 ns 1.3198 ns 1.3201 ns] |
| u32_min   | [1.1770 ns 1.1808 ns 1.1861 ns] |
| i64_max   | [10.288 ns 10.290 ns 10.292 ns] |
| i64_min   | [10.855 ns 10.865 ns 10.879 ns] |
| u64_max   | [1.3718 ns 1.3733 ns 1.3750 ns] |
| u64_min   | [1.1759 ns 1.1763 ns 1.1769 ns] |
| i128_max  | [19.546 ns 19.550 ns 19.555 ns] |
| i128_min  | [17.532 ns 17.539 ns 17.549 ns] |
| u128_max  | [13.521 ns 13.523 ns 13.526 ns] |
| u128_min  | [4.7052 ns 4.7076 ns 4.7108 ns] |
| isize_max | [10.283 ns 10.286 ns 10.288 ns] |
| isize_min | [10.868 ns 10.873 ns 10.877 ns] |
| usize_max | [1.3723 ns 1.3737 ns 1.3751 ns] |
| usize_min | [1.1758 ns 1.1760 ns 1.1763 ns] |
| true      | [1.1758 ns 1.1760 ns 1.1762 ns] |
| false     | [1.1759 ns 1.1760 ns 1.1763 ns] |
| f32_max   | [8.3351 ns 8.3541 ns 8.3808 ns] |
| f32_min   | [8.5761 ns 8.6520 ns 8.7527 ns] |
| f64_max   | [13.214 ns 13.250 ns 13.300 ns] |
| f64_min   | [13.563 ns 13.655 ns 13.744 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04

| Name              |              Time               |
|:------------------|:-------------------------------:|

## License

This software is released under the MIT License.

Third-party libraries used herein remain the property of their respective authors. Modified library code resides in the 'lib' directory. Our sincere thanks to these authors.