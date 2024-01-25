//! # Number Encoding
//!
//! ## How to Use
//!
//! Run `cargo add encoder`
//! 
//! Put trait in your code `use encoder::number::Encode;`
//!
//! ## Integer encoding
//!
//! Support i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, bool
//!
//! ```
//! use encoder::number::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     12345.encode(&mut buf);
//!     67890.encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! 
//!     println!("{}", 12345.stringify());
//!     println!("{}", 67890.stringify());
//! }
//! ```
//!
//! ## Floating point encoding
//!
//! Support f32 and f64
//!
//! ```
//! use encoder::number::Encode;
//!
//! fn main() {
//!     let mut buf = vec![];
//!     std::f32::consts::PI.encode(&mut buf);
//!     buf.push(b'\n');
//!     std::f64::consts::PI.encode(&mut buf);
//!     println!("{}", String::from_utf8_lossy(&buf));
//! 
//!     println!("{}", std::f32::consts::PI.stringify());
//!     println!("{}", std::f64::consts::PI.stringify());
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

mod float;
mod integer;