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
//! Support i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, bool, f32, f64
//!
//! ```
//! use encoder::json::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     1.encode(&mut buf);
//!     2_f32.encode(&mut buf);
//!     true.encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! ## String encoding
//!
//! Support char, &str, String
//!
//! ```
//! use encoder::json::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     "hello".encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! ## Option encoding
//!
//! Support Some(T), None
//!
//! ```
//! use encoder::json::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     let mut opt = Some(123);
//!     opt.encode(&mut buf);
//!     opt = None;
//!     opt.encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! ## Array encoding
//!
//! Support [T], Vec<T>, VecDeque<T>
//!
//! ```
//! use encoder::json::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     [1,2,3].encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! ## Object encoding
//!
//! Support HashMap, BTreeMap
//!
//! ```
//! use encoder::json::Encode;
//! use std::collections::BTreeMap;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     let mut map = BTreeMap::new();
//!     map.insert("k1", "v1");
//!     map.insert("k2", "v2");
//!     println!("{}", String::from_utf8_lossy(&buf));
//! }
//! ```
//!
//! **Happy encoding!**

pub trait Encode {
    fn encode(&self, buf: &mut Vec<u8>);

    #[inline]
    fn stringify(&self) -> String {
        let mut buf = vec![];
        self.encode(&mut buf);
        unsafe { String::from_utf8_unchecked(buf) }
    }
}

mod array;
mod number;
mod object;
mod option;
mod string;