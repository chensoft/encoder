//! # Encoder
//!
//! This library provides fast and efficient ways to encode various types into a `Vec<u8>`.
//! It avoids unnecessary allocations and copies to achieve high performance.
//!
//! ## How to Use
//!
//! Just run `cargo add encoder` to add this crate to your project, and then include the
//! relevant trait in your code like this: `use encoder::number::Encode;`
//!
//! ## Example
//!
//! Check the relevant modules.

pub mod json;
pub mod number;