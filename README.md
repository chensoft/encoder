Encoder
==========================

A collection of crates for rapid encoding

## Quick Start

#### Integer encoding

```rust
use encoder::Encode;

fn main() {
    let mut buf = vec![];
    1_i32.encode(&mut buf);
    2_u32.encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#"12"#);
}
```

#### Floating-point encoding

```rust
use encoder::Encode;

fn main() {
    let mut buf = vec![];
    1_f32.encode(&mut buf);
    2_f32.encode(&mut buf);
    3_f32.encode(&mut buf);
    1_f64.encode(&mut buf);
    2_f64.encode(&mut buf);
    3_f64.encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#"1.02.03.01.02.03.0"#);
}
```

## License

This software is released under the MIT License.

Third-party libraries used herein remain the property of their respective authors. Modified library code resides in the 'lib' directory. Our sincere thanks to these authors.