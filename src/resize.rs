use serde::Deserialize;
use serde::Serialize;

/// The method describes the way the image will be resized.
#[derive(Serialize, Deserialize)]
pub struct Method(&'static str);

impl Method {
  /// `Scales` the image down proportionally.
  pub const SCALE: &'static str = "scale";
  /// `Scales` the image down proportionally so that it `fits within` the given dimensions.
  pub const FIT: &'static str = "fit";
  /// `Scales` the image proportionally and `crops` it if necessary so that the result has exactly the given dimensions.
  pub const COVER: &'static str = "cover";
  /// A more advanced implementation of cover that also detects `cut out images` with plain backgrounds.
  pub const THUMB: &'static str = "thumb";
}

/// Use the API to create resized versions of the uploaded images.
///
/// If the `target dimensions` are larger than the original dimensions, the image will not be scaled up. Scaling up is prevented in order to protect the quality of the images.
#[derive(Serialize, Deserialize)]
pub struct Resize {
  method: String,
  pub(crate) width: Option<u32>,
  pub(crate) height: Option<u32>,
}

impl Resize {
  pub fn new<M>(method: M, width: Option<u32>, height: Option<u32>) -> Self
  where
    M: AsRef<str> + Into<String>,
  {
    Self {
      method: method.into(),
      width,
      height,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonData {
  pub(crate) resize: Resize,
}

impl JsonData {
  pub(crate) fn new(resize: Resize) -> Self {
    Self { resize }
  }
}
