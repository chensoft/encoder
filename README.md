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
| number | i8_max          | [1.1773 ns 1.1780 ns 1.1791 ns] |
| number | i8_min          | [1.1773 ns 1.1774 ns 1.1776 ns] |
| number | u8_max          | [1.1775 ns 1.1777 ns 1.1780 ns] |
| number | u8_min          | [1.1773 ns 1.1775 ns 1.1777 ns] |
| number | i16_max         | [7.3593 ns 7.3655 ns 7.3741 ns] |
| number | i16_min         | [7.6517 ns 7.6528 ns 7.6539 ns] |
| number | u16_max         | [1.1774 ns 1.1776 ns 1.1777 ns] |
| number | u16_min         | [1.1771 ns 1.1772 ns 1.1774 ns] |
| number | i32_max         | [7.0644 ns 7.0683 ns 7.0738 ns] |
| number | i32_min         | [8.5311 ns 8.5325 ns 8.5340 ns] |
| number | u32_max         | [1.3218 ns 1.3220 ns 1.3223 ns] |
| number | u32_min         | [1.1774 ns 1.1777 ns 1.1780 ns] |
| number | i64_max         | [10.301 ns 10.303 ns 10.306 ns] |
| number | i64_min         | [10.864 ns 10.871 ns 10.878 ns] |
| number | u64_max         | [1.3884 ns 1.3898 ns 1.3913 ns] |
| number | u64_min         | [1.1750 ns 1.1752 ns 1.1755 ns] |
| number | i128_max        | [19.795 ns 19.802 ns 19.809 ns] |
| number | i128_min        | [17.784 ns 17.786 ns 17.788 ns] |
| number | u128_max        | [13.505 ns 13.511 ns 13.520 ns] |
| number | u128_min        | [4.6991 ns 4.6997 ns 4.7006 ns] |
| number | isize_max       | [10.277 ns 10.282 ns 10.288 ns] |
| number | isize_min       | [10.850 ns 10.854 ns 10.857 ns] |
| number | usize_max       | [1.3721 ns 1.3728 ns 1.3736 ns] |
| number | usize_min       | [1.1742 ns 1.1743 ns 1.1743 ns] |
| number | true            | [1.1747 ns 1.1751 ns 1.1757 ns] |
| number | false           | [1.1749 ns 1.1751 ns 1.1753 ns] |
| number | f32_max         | [8.3038 ns 8.3095 ns 8.3153 ns] |
| number | f32_min         | [8.4419 ns 8.4457 ns 8.4494 ns] |
| number | f64_max         | [13.167 ns 13.169 ns 13.171 ns] |
| number | f64_min         | [13.238 ns 13.241 ns 13.244 ns] |
|  json  | char            | [8.8353 ns 8.8492 ns 8.8689 ns] |
|  json  | str             | [12.660 ns 12.662 ns 12.664 ns] |
|  json  | string          | [12.656 ns 12.658 ns 12.660 ns] |
|  json  | option          | [5.5423 ns 5.5446 ns 5.5471 ns] |
|  json  | array_raw       | [19.427 ns 19.430 ns 19.433 ns] |
|  json  | array_vec       | [16.483 ns 16.485 ns 16.488 ns] |
|  json  | array_vec_deque | [20.307 ns 20.310 ns 20.313 ns] |
|  json  | set_hash        | [19.272 ns 19.281 ns 19.295 ns] |
|  json  | set_btree       | [25.390 ns 25.400 ns 25.413 ns] |
|  json  | set_index       | [16.761 ns 16.773 ns 16.789 ns] |
|  json  | object_hash     | [54.982 ns 55.042 ns 55.124 ns] |
|  json  | object_btree    | [61.862 ns 61.924 ns 61.975 ns] |
|  json  | object_index    | [55.700 ns 55.758 ns 55.837 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 22.04

| Module | Name            |              Time               |
|:------:|-----------------|:-------------------------------:|
| number | i8_max          | [3.0645 ns 3.0648 ns 3.0650 ns] |
| number | i8_min          | [2.7861 ns 2.7862 ns 2.7863 ns] |
| number | u8_max          | [3.0649 ns 3.0650 ns 3.0651 ns] |
| number | u8_min          | [2.7877 ns 2.7879 ns 2.7881 ns] |
| number | i16_max         | [11.146 ns 11.147 ns 11.147 ns] |
| number | i16_min         | [12.263 ns 12.264 ns 12.264 ns] |
| number | u16_max         | [3.0648 ns 3.0650 ns 3.0653 ns] |
| number | u16_min         | [2.7858 ns 2.7859 ns 2.7860 ns] |
| number | i32_max         | [11.853 ns 11.860 ns 11.867 ns] |
| number | i32_min         | [13.127 ns 13.132 ns 13.137 ns] |
| number | u32_max         | [3.0660 ns 3.0662 ns 3.0663 ns] |
| number | u32_min         | [2.7874 ns 2.7876 ns 2.7878 ns] |
| number | i64_max         | [15.666 ns 15.667 ns 15.668 ns] |
| number | i64_min         | [16.215 ns 16.217 ns 16.218 ns] |
| number | u64_max         | [3.0650 ns 3.0651 ns 3.0652 ns] |
| number | u64_min         | [2.7863 ns 2.7864 ns 2.7865 ns] |
| number | i128_max        | [32.571 ns 32.573 ns 32.575 ns] |
| number | i128_min        | [32.663 ns 32.665 ns 32.667 ns] |
| number | u128_max        | [19.219 ns 19.220 ns 19.221 ns] |
| number | u128_min        | [9.4300 ns 9.4523 ns 9.4748 ns] |
| number | isize_max       | [15.665 ns 15.666 ns 15.666 ns] |
| number | isize_min       | [16.203 ns 16.204 ns 16.205 ns] |
| number | usize_max       | [3.0659 ns 3.0660 ns 3.0662 ns] |
| number | usize_min       | [2.7862 ns 2.7864 ns 2.7866 ns] |
| number | true            | [2.7860 ns 2.7861 ns 2.7862 ns] |
| number | false           | [3.0650 ns 3.0652 ns 3.0654 ns] |
| number | f32_max         | [18.247 ns 18.264 ns 18.281 ns] |
| number | f32_min         | [18.287 ns 18.306 ns 18.324 ns] |
| number | f64_max         | [25.632 ns 25.641 ns 25.650 ns] |
| number | f64_min         | [25.569 ns 25.576 ns 25.584 ns] |
|  json  | char            | [16.928 ns 16.959 ns 16.991 ns] |
|  json  | str             | [17.812 ns 17.825 ns 17.838 ns] |
|  json  | string          | [18.040 ns 18.053 ns 18.066 ns] |
|  json  | option          | [9.5428 ns 9.5551 ns 9.5672 ns] |
|  json  | array_raw       | [42.524 ns 42.558 ns 42.592 ns] |
|  json  | array_vec       | [38.338 ns 38.343 ns 38.349 ns] |
|  json  | array_vec_deque | [37.892 ns 37.894 ns 37.896 ns] |
|  json  | set_hash        | [36.211 ns 36.213 ns 36.215 ns] |
|  json  | set_btree       | [45.391 ns 45.416 ns 45.442 ns] |
|  json  | set_index       | [34.817 ns 34.818 ns 34.819 ns] |
|  json  | object_hash     | [84.059 ns 84.226 ns 84.389 ns] |
|  json  | object_btree    | [85.961 ns 86.056 ns 86.156 ns] |
|  json  | object_index    | [72.589 ns 72.626 ns 72.660 ns] |

## License

This software is released under the MIT License.

Third-party libraries used herein remain the property of their respective authors. Modified library code resides in the 'lib' directory. Our sincere thanks to these authors.