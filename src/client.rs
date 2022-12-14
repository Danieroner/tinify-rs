use crate::error::TinifyError;
use crate::source::Source;
use std::path::Path;

/// The Tinify Client.
pub struct Client {
  key: String,
}

impl Client {
  pub(crate) fn new<K>(key: K) -> Self
  where
    K: AsRef<str> + Into<String>,
  {
    Self { 
      key: key.into(),
    }
  }

  fn get_source(&self) -> Source {
    Source::new(None, Some(self.key.as_str())) 
  }

  /// Choose a file to compress.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::{Tinify, TinifyException};
  /// 
  /// fn main() -> Result<(), TinifyException> {
  ///   let key = "tinify api key";
  ///   let optimized = Tinify::new()
  ///     .set_key(key)
  ///     .get_client()?
  ///     .from_file("./unoptimized.png")?
  ///     .to_file("./optimized.png");
  ///   
  ///   Ok(())
  /// }
  /// ```
  pub fn from_file<P>(
    &self,
    path: P,
  ) -> Result<Source, TinifyError>
  where
    P: AsRef<Path>,
  {
    self
      .get_source()
      .from_file(path)
  }

  /// Choose a buffer to compress.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::{Tinify, TinifyException};
  /// use std::fs;
  /// 
  /// fn main() -> Result<(), TinifyException> {
  ///   let key = "tinify api key";
  ///   let bytes = fs::read("./unoptimized.png").unwrap();
  ///   let buffer = Tinify::new()
  ///     .set_key(key)
  ///     .get_client()?
  ///     .from_buffer(&bytes)?
  ///     .to_buffer();
  ///  
  ///   let save = fs::write("./optimized.png", buffer).unwrap();
  ///   
  ///   Ok(())
  /// }
  /// ```
  pub fn from_buffer(
    &self,
    buffer: &[u8],
  ) -> Result<Source, TinifyError> {
    self
      .get_source()
      .from_buffer(buffer)
  }

  /// Choose an url file to compress.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::{Tinify, TinifyException};
  /// 
  /// fn main() -> Result<(), TinifyException> {
  ///   let key = "tinify api key";
  ///   let optimized = Tinify::new()
  ///     .set_key(key)
  ///     .get_client()?
  ///     .from_url("https://tinypng.com/images/panda-happy.png")?
  ///     .to_file("./optimized.png");
  /// 
  ///   
  ///   Ok(())
  /// }
  /// ```
  pub fn from_url<P>(
    &self,
    url: P,
  ) -> Result<Source, TinifyError>
  where
    P: AsRef<str>,
  {
    self
      .get_source()
      .from_url(url)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::resize::ResizeMethod;
  use crate::resize::Resize;
  use crate::TinifyError;
  use reqwest::blocking::Client as ReqwestClient;
  use assert_matches::assert_matches;
  use imagesize::size;
  use dotenv::dotenv;
  use std::env;
  use std::fs;

  fn get_key() -> String {
    dotenv().ok();
    let key = match env::var("KEY") {
      Ok(key) => key,
      Err(_err) => panic!("No such file or directory."),
    };
  
    key
  }

  #[test]
  fn test_invalid_key() {
    let client = Client::new("invalid");
    let request = client
      .from_url("https://tinypng.com/images/panda-happy.png")
      .unwrap_err();

    assert_matches!(request, TinifyError::ClientError);
  }

  #[test]
  fn test_compress_from_file() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let client = Client::new(key);
    let _ = client.from_file(tmp_image)?.to_file(output)?;
    let actual = fs::metadata(tmp_image)?.len();
    let expected = fs::metadata(output)?.len();

    assert_eq!(actual, 124814);
    assert_eq!(expected, 102051);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_compress_from_buffer() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let buffer = fs::read(tmp_image).unwrap();
    let client = Client::new(key);
    let _ = client.from_buffer(&buffer)?.to_file(output)?;
    let actual = fs::metadata(tmp_image)?.len();
    let expected = fs::metadata(output)?.len();

    assert_eq!(actual, 124814);
    assert_eq!(expected, 102051);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_compress_from_url() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let remote_image = "https://tinypng.com/images/panda-happy.png";
    let client = Client::new(key);
    let _ = client.from_url(remote_image)?.to_file(output)?;
    let expected = fs::metadata(output)?.len();

    let actual = ReqwestClient::new()
      .get(remote_image)
      .send()?;

    if let Some(content_length) = actual.content_length() {
      assert_eq!(content_length, 30734);
    }

    assert_eq!(expected, 26324);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_save_to_file() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let client = Client::new(key);
    let _ = client.from_file(tmp_image)?.to_file(output)?;

    assert!(output.exists());

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_save_to_bufffer() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let client = Client::new(key);
    let buffer = client.from_file(tmp_image)?.to_buffer();

    assert_eq!(buffer.capacity(), 102051);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_resize_scale_width() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg".to_string())?
      .resize(Resize::new(ResizeMethod::SCALE, Some(400), None))?
      .to_file(output)?;

    let (width, height) = match size(output) {
      Ok(dim) => (dim.width, dim.height),
      Err(err) => panic!("Error getting dimensions: {:?}", err),
    };

    assert_eq!((width, height), (400, 200));

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_resize_scale_height() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg".to_string())?
      .resize(Resize::new(ResizeMethod::SCALE, None, Some(400)))?
      .to_file(output)?;

    let (width, height) = match size(output) {
      Ok(dim) => (dim.width, dim.height),
      Err(err) => panic!("Error getting dimensions: {:?}", err),
    };

    assert_eq!((width, height), (800, 400));

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_resize_fit() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg".to_string())?
      .resize(Resize::new(ResizeMethod::FIT, Some(400), Some(200)))?
      .to_file(output)?;

    let (width, height) = match size(output) {
      Ok(dim) => (dim.width, dim.height),
      Err(err) => panic!("Error getting dimensions: {:?}", err),
    };

    assert_eq!((width, height), (400, 200));

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_resize_cover() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg".to_string())?
      .resize(Resize::new(ResizeMethod::COVER, Some(400), Some(200)))?
      .to_file(output)?;

    let (width, height) = match size(output) {
      Ok(dim) => (dim.width, dim.height),
      Err(err) => panic!("Error getting dimensions: {:?}", err),
    };

    assert_eq!((width, height), (400, 200));

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[test]
  fn test_resize_thumb() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg".to_string())?
      .resize(Resize::new(ResizeMethod::THUMB, Some(400), Some(200)))?
      .to_file(output)?;

    let (width, height) = match size(output) {
      Ok(dim) => (dim.width, dim.height),
      Err(err) => panic!("Error getting dimensions: {:?}", err),
    };

    assert_eq!((width, height), (400, 200));

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }
}
