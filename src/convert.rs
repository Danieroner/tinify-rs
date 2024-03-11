use serde::Deserialize;
use serde::Serialize;

/// Tinify currently supports converting between WebP, JPEG, and PNG.
///
/// When provided more than one image type in the convert request,
/// the smallest version will be returned.
#[derive(Serialize, Deserialize)]
pub struct Type(&'static str);

#[allow(dead_code)]
impl Type {
  /// The `"image/png"` type.
  pub const PNG: &'static str = "image/png";
  /// The `"image/jpeg"` type.
  pub const JPEG: &'static str = "image/jpeg";
  /// The `"image/webp"` type.
  pub const WEBP: &'static str = "image/webp";
  /// The wildcard `"*/*"` returns the smallest of Tinify's supported image types,
  /// currently WebP, JPEG and PNG.
  pub const WILDCARD: &'static str = "*/*";
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Convert {
  convert_type: String,
}

impl Convert {
  pub(crate) fn new<C>(convert_type: C) -> Self
  where
    C: Into<String>,
  {
    Self {
      convert_type: convert_type.into(),
    }
  }
}

/// A hex value. Custom background color using the color's hex value: `"#000000"`.
/// `white` or `black`. Only the colors white and black are supported as strings.
#[derive(Serialize, Deserialize, Debug)]
pub struct Color(pub &'static str);

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transform {
  background: String,
}

impl Transform {
  pub(crate) fn new<B>(background: B) -> Self
  where
    B: AsRef<str> + Into<String>,
  {
    Self {
      background: background.into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct JsonData {
  pub(crate) convert: Convert,
  transform: Option<Transform>,
}

impl JsonData {
  pub(crate) fn new(convert: Convert, transform: Option<Transform>) -> Self {
    Self { convert, transform }
  }
}
