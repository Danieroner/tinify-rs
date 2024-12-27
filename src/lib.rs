//! # Tinify Crate
//!
//! `tinify-rs` is a Rust client for the Tinify API.
//! Used for TinyPNG and TinyJPG. Tinify compresses your images intelligently.
//!
//! Read more at `https://tinify.com`
// --snip--

use convert::Convert;
use resize::Resize;
use serde::Deserialize;
use serde::Serialize;
use transform::Transform;

#[cfg(feature = "async")]
pub mod async_bin;
pub mod convert;
pub mod error;
pub mod resize;
#[cfg(not(feature = "async"))]
pub mod sync;
pub mod transform;

pub(crate) const API_ENDPOINT: &str = "https://api.tinify.com";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SourceUrl {
  url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Operations {
  #[serde(skip_serializing_if = "Option::is_none")]
  convert: Option<Convert>,

  #[serde(skip_serializing_if = "Option::is_none")]
  resize: Option<Resize>,

  #[serde(skip_serializing_if = "Option::is_none")]
  transform: Option<Transform>,
}
