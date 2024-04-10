use serde::Deserialize;
use serde::Serialize;

/// The transform object specifies the stylistic transformations that will be applied to your image. Include a `background property` to fill a transparent image's background. The following options are available to specify a background color:
/// - A hex value. Custom background color using the color's hex value: `#000000`.
/// - `white` or `black`. Only the colors white and black are supported as strings.
///
/// You must specify a background color if you wish to convert an image with a transparent background to an image type which does not support transparency (like JPEG).
#[derive(Serialize, Deserialize, Debug)]
pub struct Transform {
  pub background: String,
}
