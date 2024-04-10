use serde::Deserialize;
use serde::Serialize;

/// The type `enum` defines the type of image to which it will be converted.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Type {
  #[serde(rename = "image/png")]
  Png,

  #[serde(rename = "image/jpeg")]
  Jpeg,

  #[serde(rename = "image/webp")]
  Webp,

  #[serde(rename = "*/*")]
  WildCard,
}

/// # Converting images
///
/// You can use the API to convert your images to your desired image type. Tinify currently supports converting between `WebP`, J`PEG`, and `PNG`. When you provide more than one image `type` in your convert request, the smallest version will be returned to you.
///
/// Image converting will count as one additional compression.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Convert {
  /// A vector of `types`
  pub r#type: Vec<Type>,
}
