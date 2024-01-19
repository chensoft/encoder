//! # Encoder
//!
//! This library provides fast and efficient ways to encode various types into a `Vec<u8>`.
//! It avoids unnecessary allocations and copies to achieve high performance.
//!
//! ## How to Use
//!
//! Just run `cargo add encoder` to add this crate to your project, and then include the trait
//! in your code like this: `use encoder::Encode;`
//!
//! ## Integer encoding
//!
//! Support i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, bool
//!
//! ```
//! use encoder::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     12345.encode(&mut buf);
//!     67890.encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! ## Floating point encoding
//!
//! Support f32 and f64
//!
//! ```
//! use encoder::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     std::f32::consts::PI.encode(&mut buf);
//!     buf.push(b'\n');
//!     std::f64::consts::PI.encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! **Happy encoding!**

pub trait Encode: Sync + Send {
    fn encode(&self, buf: &mut Vec<u8>);
}

mod float;
mod integer;