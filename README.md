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

## License

This software is released under the MIT License.

Third-party libraries used herein remain the property of their respective authors. Modified library code resides in the 'lib' directory. Our sincere thanks to these authors.