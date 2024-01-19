//! # Json Encoding
//!
//! ## How to Use
//!
//! Run `cargo add encoder`
//!
//! Put trait in your code `use encoder::json::Encode;`
//!
//! ## Number encoding
//!
//! Support i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, bool
//!
//! ```
//! use encoder::json::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! **Happy encoding!**

pub trait Encode: Sync + Send {
    fn encode(&self, buf: &mut Vec<u8>);
}

mod array;
mod number;
mod object;
mod option;
mod string;