Encoder
==========================

A blazing-fast encoder writing to a contiguous buffer

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

- MacBook Air, Apple M2 24G, Sonoma 14.5, Rust 1.78.0

| Module | Name            |              Time               |
|:------:|-----------------|:-------------------------------:|
| number | i8_max          | [1.1760 ns 1.1762 ns 1.1764 ns] |
| number | i8_min          | [1.1761 ns 1.1763 ns 1.1765 ns] |
| number | u8_max          | [1.1761 ns 1.1763 ns 1.1764 ns] |
| number | u8_min          | [1.1758 ns 1.1760 ns 1.1762 ns] |
| number | i16_max         | [1.1761 ns 1.1763 ns 1.1765 ns] |
| number | i16_min         | [1.1758 ns 1.1760 ns 1.1762 ns] |
| number | u16_max         | [1.1760 ns 1.1762 ns 1.1764 ns] |
| number | u16_min         | [1.1758 ns 1.1768 ns 1.1788 ns] |
| number | i32_max         | [7.0556 ns 7.0572 ns 7.0590 ns] |
| number | i32_min         | [8.5205 ns 8.5222 ns 8.5239 ns] |
| number | u32_max         | [1.3197 ns 1.3199 ns 1.3201 ns] |
| number | u32_min         | [1.1760 ns 1.1782 ns 1.1827 ns] |
| number | i64_max         | [10.573 ns 10.574 ns 10.576 ns] |
| number | i64_min         | [10.869 ns 10.874 ns 10.878 ns] |
| number | u64_max         | [1.3756 ns 1.3770 ns 1.3783 ns] |
| number | u64_min         | [1.1759 ns 1.1761 ns 1.1763 ns] |
| number | i128_max        | [19.427 ns 19.431 ns 19.435 ns] |
| number | i128_min        | [17.261 ns 17.264 ns 17.267 ns] |
| number | u128_max        | [13.814 ns 13.821 ns 13.832 ns] |
| number | u128_min        | [4.9977 ns 4.9984 ns 4.9993 ns] |
| number | isize_max       | [10.574 ns 10.575 ns 10.577 ns] |
| number | isize_min       | [10.864 ns 10.869 ns 10.874 ns] |
| number | usize_max       | [1.3742 ns 1.3754 ns 1.3766 ns] |
| number | usize_min       | [1.1759 ns 1.1761 ns 1.1763 ns] |
| number | true            | [1.1758 ns 1.1760 ns 1.1763 ns] |
| number | false           | [1.1758 ns 1.1760 ns 1.1761 ns] |
| number | f32_max         | [8.1414 ns 8.1446 ns 8.1478 ns] |
| number | f32_min         | [8.3086 ns 8.3100 ns 8.3113 ns] |
| number | f64_max         | [12.451 ns 12.453 ns 12.456 ns] |
| number | f64_min         | [12.265 ns 12.270 ns 12.274 ns] |
|  json  | char            | [8.9673 ns 8.9782 ns 8.9970 ns] |
|  json  | str             | [11.769 ns 11.789 ns 11.815 ns] |
|  json  | string          | [11.749 ns 11.752 ns 11.755 ns] |
|  json  | option          | [5.8269 ns 5.8281 ns 5.8294 ns] |
|  json  | array_raw       | [20.288 ns 20.293 ns 20.298 ns] |
|  json  | array_vec       | [16.904 ns 16.907 ns 16.910 ns] |
|  json  | array_vec_deque | [19.699 ns 19.736 ns 19.815 ns] |
|  json  | set_hash        | [20.348 ns 20.358 ns 20.371 ns] |
|  json  | set_btree       | [23.813 ns 23.818 ns 23.823 ns] |
|  json  | set_index       | [17.163 ns 17.166 ns 17.169 ns] |
|  json  | object_hash     | [49.057 ns 49.400 ns 49.737 ns] |
|  json  | object_btree    | [52.801 ns 52.909 ns 53.052 ns] |
|  json  | object_index    | [48.631 ns 48.639 ns 48.648 ns] |

- AWS c5.2xlarge, 8C 16G, Ubuntu 24.04, Rust 1.78.0

| Module | Name            |              Time               |
|:------:|-----------------|:-------------------------------:|
| number | i8_max          | [3.2681 ns 3.2683 ns 3.2684 ns] |
| number | i8_min          | [2.9715 ns 2.9717 ns 2.9720 ns] |
| number | u8_max          | [3.2682 ns 3.2684 ns 3.2686 ns] |
| number | u8_min          | [2.9710 ns 2.9712 ns 2.9713 ns] |
| number | i16_max         | [3.2683 ns 3.2686 ns 3.2688 ns] |
| number | i16_min         | [3.2684 ns 3.2686 ns 3.2688 ns] |
| number | u16_max         | [3.2708 ns 3.2711 ns 3.2714 ns] |
| number | u16_min         | [2.9751 ns 2.9754 ns 2.9758 ns] |
| number | i32_max         | [12.834 ns 12.849 ns 12.864 ns] |
| number | i32_min         | [12.876 ns 12.885 ns 12.894 ns] |
| number | u32_max         | [3.2686 ns 3.2688 ns 3.2691 ns] |
| number | u32_min         | [2.9710 ns 2.9711 ns 2.9713 ns] |
| number | i64_max         | [16.837 ns 16.848 ns 16.858 ns] |
| number | i64_min         | [17.232 ns 17.250 ns 17.268 ns] |
| number | u64_max         | [3.2680 ns 3.2682 ns 3.2684 ns] |
| number | u64_min         | [2.9712 ns 2.9714 ns 2.9716 ns] |
| number | i128_max        | [34.801 ns 34.808 ns 34.815 ns] |
| number | i128_min        | [34.452 ns 34.456 ns 34.460 ns] |
| number | u128_max        | [20.215 ns 20.217 ns 20.219 ns] |
| number | u128_min        | [11.169 ns 11.183 ns 11.195 ns] |
| number | isize_max       | [17.001 ns 17.009 ns 17.017 ns] |
| number | isize_min       | [17.310 ns 17.319 ns 17.329 ns] |
| number | usize_max       | [3.2680 ns 3.2682 ns 3.2684 ns] |
| number | usize_min       | [2.9710 ns 2.9712 ns 2.9713 ns] |
| number | true            | [2.9709 ns 2.9710 ns 2.9712 ns] |
| number | false           | [3.2680 ns 3.2682 ns 3.2685 ns] |
| number | f32_max         | [18.512 ns 18.522 ns 18.533 ns] |
| number | f32_min         | [18.409 ns 18.413 ns 18.417 ns] |
| number | f64_max         | [26.787 ns 26.790 ns 26.793 ns] |
| number | f64_min         | [26.948 ns 26.955 ns 26.962 ns] |
|  json  | char            | [19.684 ns 19.714 ns 19.745 ns] |
|  json  | str             | [19.283 ns 19.307 ns 19.330 ns] |
|  json  | string          | [19.306 ns 19.324 ns 19.341 ns] |
|  json  | option          | [8.6175 ns 8.6182 ns 8.6188 ns] |
|  json  | array_raw       | [42.462 ns 42.471 ns 42.480 ns] |
|  json  | array_vec       | [35.654 ns 35.656 ns 35.659 ns] |
|  json  | array_vec_deque | [42.748 ns 42.789 ns 42.832 ns] |
|  json  | set_hash        | [40.382 ns 40.398 ns 40.415 ns] |
|  json  | set_btree       | [48.006 ns 48.014 ns 48.022 ns] |
|  json  | set_index       | [39.517 ns 39.519 ns 39.521 ns] |
|  json  | object_hash     | [86.702 ns 86.777 ns 86.851 ns] |
|  json  | object_btree    | [93.088 ns 93.129 ns 93.172 ns] |
|  json  | object_index    | [86.768 ns 86.828 ns 86.891 ns] |

## Documentation

The documentation is [available here](https://docs.rs/encoder).

## License

This software is released under the [MIT License](https://github.com/chensoft/encoder?tab=MIT-1-ov-file).