use serde::Deserialize;
use serde::Serialize;

/// The method describes the way your image will be resized. The following methods are available:
#[derive(Serialize, Deserialize, Debug)]
pub enum Method {
  /// Scales the image down proportionally. You must provide either a target `width` or a target `height`, but not both. The scaled image will have exactly the provided width or height.
  #[serde(rename = "scale")]
  Scale,

  /// Scales the image down proportionally so that it fits within the given dimensions. You must provide both a `width` and a `height`. The scaled image will not exceed either of these dimensions.
  #[serde(rename = "fit")]
  Fit,

  /// Scales the image proportionally and crops it if necessary so that the result has exactly the given dimensions. You must provide both a `width` and a `height`. Which parts of the image are cropped away is determined automatically. An intelligent algorithm determines the most important areas of your image.
  #[serde(rename = "cover")]
  Cover,

  /// A more advanced implementation of cover that also detects cut out images with plain backgrounds. The image is scaled down to the `width` and `height` you provide. If an image is detected with a free standing object it will add more background space where necessary or crop the unimportant parts.
  #[serde(rename = "thumb")]
  Thumb,
}

/// # Resizing images
/// Use the API to create resized versions of your uploaded images. By letting the API handle resizing you avoid having to write such code yourself and you will only have to upload your image once. The resized images will be optimally compressed with a nice and crisp appearance.
///
/// You can also take advantage of intelligent cropping to create thumbnails that focus on the most visually important areas of your image.
///
/// Resizing counts as one additional compression. For example, if you upload a single image and retrieve the optimized version plus 2 resized versions this will count as 3 compressions in total.
#[derive(Serialize, Deserialize, Debug)]
pub struct Resize {
  pub method: Method,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub width: Option<u32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub height: Option<u32>,
}
