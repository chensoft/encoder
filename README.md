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

- MacBook Air, Apple M2 24G, Sonoma 14.2.1

| Module | Name            |              Time               |
|:------:|-----------------|:-------------------------------:|
| number | i8_max          | [1.1761 ns 1.1767 ns 1.1775 ns] |
| number | i8_min          | [1.1752 ns 1.1754 ns 1.1757 ns] |
| number | u8_max          | [1.1759 ns 1.1761 ns 1.1764 ns] |
| number | u8_min          | [1.1759 ns 1.1761 ns 1.1763 ns] |
| number | i16_max         | [8.0300 ns 8.0445 ns 8.0584 ns] |
| number | i16_min         | [8.1324 ns 8.1535 ns 8.1812 ns] |
| number | u16_max         | [1.4697 ns 1.4698 ns 1.4700 ns] |
| number | u16_min         | [1.1757 ns 1.1759 ns 1.1762 ns] |
| number | i32_max         | [7.0862 ns 7.0872 ns 7.0883 ns] |
| number | i32_min         | [8.5705 ns 8.6005 ns 8.6398 ns] |
| number | u32_max         | [1.3195 ns 1.3198 ns 1.3201 ns] |
| number | u32_min         | [1.1770 ns 1.1808 ns 1.1861 ns] |
| number | i64_max         | [10.288 ns 10.290 ns 10.292 ns] |
| number | i64_min         | [10.855 ns 10.865 ns 10.879 ns] |
| number | u64_max         | [1.3718 ns 1.3733 ns 1.3750 ns] |
| number | u64_min         | [1.1759 ns 1.1763 ns 1.1769 ns] |
| number | i128_max        | [19.546 ns 19.550 ns 19.555 ns] |
| number | i128_min        | [17.532 ns 17.539 ns 17.549 ns] |
| number | u128_max        | [13.521 ns 13.523 ns 13.526 ns] |
| number | u128_min        | [4.7052 ns 4.7076 ns 4.7108 ns] |
| number | isize_max       | [10.283 ns 10.286 ns 10.288 ns] |
| number | isize_min       | [10.868 ns 10.873 ns 10.877 ns] |
| number | usize_max       | [1.3723 ns 1.3737 ns 1.3751 ns] |
| number | usize_min       | [1.1758 ns 1.1760 ns 1.1763 ns] |
| number | true            | [1.1758 ns 1.1760 ns 1.1762 ns] |
| number | false           | [1.1759 ns 1.1760 ns 1.1763 ns] |
| number | f32_max         | [8.3351 ns 8.3541 ns 8.3808 ns] |
| number | f32_min         | [8.5761 ns 8.6520 ns 8.7527 ns] |
| number | f64_max         | [13.214 ns 13.250 ns 13.300 ns] |
| number | f64_min         | [13.563 ns 13.655 ns 13.744 ns] |
|  json  | char            | [6.4216 ns 6.4337 ns 6.4483 ns] |
|  json  | str             | [13.003 ns 13.151 ns 13.319 ns] |
|  json  | string          | [12.717 ns 12.776 ns 12.849 ns] |
|  json  | option          | [6.7864 ns 6.8134 ns 6.8431 ns] |
|  json  | array_raw       | [23.851 ns 23.895 ns 23.955 ns] |
|  json  | array_vec       | [16.519 ns 16.547 ns 16.585 ns] |
|  json  | array_vec_deque | [20.612 ns 21.718 ns 23.281 ns] |
|  json  | set_hash        | [19.402 ns 19.498 ns 19.622 ns] |
|  json  | set_btree       | [25.521 ns 25.752 ns 26.026 ns] |
|  json  | set_index       | [16.864 ns 16.914 ns 16.978 ns] |
|  json  | object_hash     | [57.765 ns 58.195 ns 58.669 ns] |
|  json  | object_btree    | [62.765 ns 63.046 ns 63.337 ns] |
|  json  | object_index    | [56.811 ns 57.271 ns 57.810 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04

| Module | Name            |              Time               |
|:------:|-----------------|:-------------------------------:|
| number | i8_max          | [3.0653 ns 3.0656 ns 3.0661 ns] |
| number | i8_min          | [2.7852 ns 2.7853 ns 2.7854 ns] |
| number | u8_max          | [3.0634 ns 3.0635 ns 3.0637 ns] |
| number | u8_min          | [2.7856 ns 2.7859 ns 2.7863 ns] |
| number | i16_max         | [11.146 ns 11.146 ns 11.147 ns] |
| number | i16_min         | [12.272 ns 12.273 ns 12.274 ns] |
| number | u16_max         | [3.0640 ns 3.0643 ns 3.0646 ns] |
| number | u16_min         | [2.7854 ns 2.7857 ns 2.7861 ns] |
| number | i32_max         | [11.994 ns 11.998 ns 12.002 ns] |
| number | i32_min         | [13.939 ns 13.940 ns 13.941 ns] |
| number | u32_max         | [3.0633 ns 3.0634 ns 3.0635 ns] |
| number | u32_min         | [2.7852 ns 2.7853 ns 2.7855 ns] |
| number | i64_max         | [15.952 ns 15.957 ns 15.963 ns] |
| number | i64_min         | [16.437 ns 16.438 ns 16.439 ns] |
| number | u64_max         | [3.0642 ns 3.0644 ns 3.0646 ns] |
| number | u64_min         | [2.7852 ns 2.7854 ns 2.7857 ns] |
| number | i128_max        | [32.384 ns 32.387 ns 32.391 ns] |
| number | i128_min        | [32.707 ns 32.711 ns 32.716 ns] |
| number | u128_max        | [19.774 ns 19.776 ns 19.778 ns] |
| number | u128_min        | [9.1635 ns 9.1670 ns 9.1705 ns] |
| number | isize_max       | [15.943 ns 15.947 ns 15.952 ns] |
| number | isize_min       | [16.446 ns 16.451 ns 16.459 ns] |
| number | usize_max       | [3.0647 ns 3.0651 ns 3.0655 ns] |
| number | usize_min       | [2.7860 ns 2.7863 ns 2.7866 ns] |
| number | true            | [2.7849 ns 2.7850 ns 2.7850 ns] |
| number | false           | [3.0634 ns 3.0644 ns 3.0657 ns] |
| number | f32_max         | [18.413 ns 18.436 ns 18.459 ns] |
| number | f32_min         | [18.370 ns 18.382 ns 18.395 ns] |
| number | f64_max         | [25.477 ns 25.483 ns 25.488 ns] |
| number | f64_min         | [25.473 ns 25.477 ns 25.480 ns] |
|  json  | char            | [10.996 ns 10.996 ns 10.997 ns] |
|  json  | str             | [21.990 ns 21.994 ns 21.997 ns] |
|  json  | string          | [22.347 ns 22.362 ns 22.377 ns] |
|  json  | option          | [9.8923 ns 9.9044 ns 9.9168 ns] |
|  json  | array_raw       | [45.982 ns 45.985 ns 45.988 ns] |
|  json  | array_vec       | [35.649 ns 35.651 ns 35.655 ns] |
|  json  | array_vec_deque | [41.837 ns 41.849 ns 41.861 ns] |
|  json  | set_hash        | [37.887 ns 37.892 ns 37.896 ns] |
|  json  | set_btree       | [61.903 ns 61.945 ns 61.981 ns] |
|  json  | set_index       | [35.652 ns 35.655 ns 35.660 ns] |
|  json  | object_hash     | [89.120 ns 89.157 ns 89.203 ns] |
|  json  | object_btree    | [107.65 ns 107.74 ns 107.82 ns] |
|  json  | object_index    | [86.747 ns 86.799 ns 86.850 ns] |

## License

This software is released under the MIT License.

Third-party libraries used herein remain the property of their respective authors. Modified library code resides in the 'lib' directory. Our sincere thanks to these authors.