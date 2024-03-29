//! # Encoder
//!
//! This library provides fast and efficient ways to encode various types into a `Vec<u8>`.
//! It avoids unnecessary allocations and copies to achieve high performance.
//!
//! ## How to Use
//!
//! Run `cargo add encoder`
//!
//! Put module trait in your code `use encoder::<MODULE>::Encode;`
//!
//! ## Example
//!
//! Check the relevant modules.
#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::let_underscore_future)]

pub mod json;
pub mod number;