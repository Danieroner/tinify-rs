//! # Tinify Crate
//!
//! `tinify-rs` is a Rust client for the Tinify API.
//! Used for TinyPNG and TinyJPG. Tinify compresses your images intelligently.
//!
//! Read more at `https://tinify.com`
// --snip--

#[cfg(feature = "async")]
pub mod async_bin;
pub mod convert;
pub mod error;
pub mod resize;
pub mod sync;

pub (crate) const API_ENDPOINT: &str = "https://api.tinify.com";
