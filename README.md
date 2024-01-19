Encoder
==========================

A collection of crates for rapid encoding

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
}
```

#### Json encoding

```rust
use encoder::json::Encode;

fn main() {
    let mut buf = vec![];
    assert_eq!(String::from_utf8_lossy(&buf), r#""#);
}
```

## License

This software is released under the MIT License.

Third-party libraries used herein remain the property of their respective authors. Modified library code resides in the 'lib' directory. Our sincere thanks to these authors.