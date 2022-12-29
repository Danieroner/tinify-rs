use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Type(&'static str);

impl Type {
  pub const PNG: &'static str = "image/png";
  pub const JPEG: &'static str = "image/jpeg";
  pub const WEBP: &'static str = "image/webp";
  pub const WILDCARD: &'static str = "*/*";
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Convert {
  convert_type: String,
}

impl Convert {
  pub fn new<C>(convert_type: C) -> Self
  where
    C: AsRef<str> + Into<String>,
  {
    Self {
      convert_type: convert_type.into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color(pub &'static str);

#[derive(Serialize, Deserialize, Debug)]
pub struct Transform {
  background: String,
}

impl Transform {
  pub fn new<B>(background: B) -> Self
  where
    B: AsRef<str> + Into<String>,
  {
    Self {
      background: background.into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData {
  pub(crate) convert: Convert,
  transform: Option<Transform>,
}

impl JsonData {
  pub(crate) fn new(
    convert: Convert,
    transform: Option<Transform>,
  )-> Self {
    Self { 
      convert,
      transform,
    }
  }
}
