use crate::async_bin::source::Source;
use crate::error::TinifyError;
use std::path::Path;

/// The Tinify Client.
pub struct Client {
  source: Source,
}

impl Client {
  pub(crate) fn new<K>(key: K) -> Self
  where
    K: AsRef<str>,
  {
    Self {
      source: Source::new(None, Some(key.as_ref())),
    }
  }

  /// Choose a file to compress.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::async_bin::Tinify as AsyncTinify;
  /// use tinify::error::TinifyError;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), TinifyError> {
  ///   let key = "tinify api key";
  ///   let tinify = AsyncTinify::new().set_key(key);
  ///   let client = tinify.get_async_client()?;
  ///   client
  ///     .from_file("./unoptimized.jpg")
  ///     .await?
  ///     .to_file("./optimized.jpg")?;
  ///
  ///   Ok(())
  /// }
  /// ```
  #[allow(clippy::wrong_self_convention)]
  pub async fn from_file<P>(self, path: P) -> Result<Source, TinifyError>
  where
    P: AsRef<Path>,
  {
    self.source.from_file(path).await
  }

  /// Choose a buffer to compress.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::async_bin::Tinify as AsyncTinify;
  /// use tinify::error::TinifyError;
  /// use std::fs;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), TinifyError> {
  ///   let key = "tinify api key";
  ///   let tinify = AsyncTinify::new().set_key(key);
  ///   let bytes = fs::read("./unoptimized.jpg")?;
  ///   let client = tinify.get_async_client()?;
  ///   client
  ///     .from_buffer(&bytes)
  ///     .await?
  ///     .to_file("./optimized.jpg")?;
  ///
  ///   Ok(())
  /// }
  /// ```
  #[allow(clippy::wrong_self_convention)]
  pub async fn from_buffer(self, buffer: &[u8]) -> Result<Source, TinifyError> {
    self.source.from_buffer(buffer).await
  }

  /// Choose an url image to compress.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::async_bin::Tinify as AsyncTinify;
  /// use tinify::error::TinifyError;
  ///
  /// #[tokio::main]
  /// async fn main() -> Result<(), TinifyError> {
  ///   let key = "tinify api key";
  ///   let tinify = AsyncTinify::new().set_key(key);
  ///   let client = tinify.get_async_client()?;
  ///   client
  ///     .from_url("https://tinypng.com/images/panda-happy.png")
  ///     .await?
  ///     .to_file("./optimized.jpg")?;
  ///
  ///   Ok(())
  /// }
  /// ```
  #[allow(clippy::wrong_self_convention)]
  pub async fn from_url<P>(self, url: P) -> Result<Source, TinifyError>
  where
    P: AsRef<str>,
  {
    self.source.from_url(url).await
  }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
  use super::*;
  use crate::convert::Color;
  use crate::convert::Type;
  use crate::resize::Method;
  use crate::resize::Resize;
  use assert_matches::assert_matches;
  use dotenv::dotenv;
  use imagesize::size;
  use reqwest::Client as ReqwestClient;
  use std::env;
  use std::ffi::OsStr;
  use std::fs;

  fn get_key() -> String {
    dotenv().ok();
    match env::var("KEY") {
      Ok(key) => key,
      Err(_err) => panic!("No such file or directory."),
    }
  }

  #[tokio::test]
  async fn test_invalid_key() {
    let client = Client::new("invalid");
    let request = client
      .from_url("https://tinypng.com/images/panda-happy.png")
      .await
      .unwrap_err();

    assert_matches!(request, TinifyError::ClientError);
  }

  #[tokio::test]
  async fn test_compress_from_file() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let _ = Client::new(key).from_file(tmp_image).await?.to_file(output);
    let actual = fs::metadata(tmp_image)?.len();
    let expected = fs::metadata(output)?.len();

    assert_eq!(actual, 124814);
    assert_eq!(expected, 102051);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_compress_from_buffer() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let buffer = fs::read(tmp_image).unwrap();
    let _ = Client::new(key).from_buffer(&buffer).await?.to_file(output);
    let actual = fs::metadata(tmp_image)?.len();
    let expected = fs::metadata(output)?.len();

    assert_eq!(actual, 124814);
    assert_eq!(expected, 102051);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_compress_from_url() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let remote_image = "https://tinypng.com/images/panda-happy.png";
    let _ = Client::new(key)
      .from_url(remote_image)
      .await?
      .to_file(output);
    let expected = fs::metadata(output)?.len();

    let actual = ReqwestClient::new().get(remote_image).send().await?;

    if let Some(content_length) = actual.content_length() {
      assert_eq!(content_length, 30734);
    }

    assert_eq!(expected, 26324);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_save_to_file() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let _ = Client::new(key).from_file(tmp_image).await?.to_file(output);

    assert!(output.exists());

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_save_to_bufffer() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./optimized.jpg");
    let tmp_image = Path::new("./tmp_image.jpg");
    let client = Client::new(key);
    let buffer = client.from_file(tmp_image).await?.to_buffer();

    assert_eq!(buffer.capacity(), 102051);

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_resize_scale_width() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg")
      .await?
      .resize(Resize::new(Method::SCALE, Some(400), None))
      .await?
      .to_file(output);

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

  #[tokio::test]
  async fn test_resize_scale_height() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg")
      .await?
      .resize(Resize::new(Method::SCALE, None, Some(400)))
      .await?
      .to_file(output);

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

  #[tokio::test]
  async fn test_resize_fit() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg")
      .await?
      .resize(Resize::new(Method::FIT, Some(400), Some(200)))
      .await?
      .to_file(output);

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

  #[tokio::test]
  async fn test_resize_cover() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg")
      .await?
      .resize(Resize::new(Method::COVER, Some(400), Some(200)))
      .await?
      .to_file(output);

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

  #[tokio::test]
  async fn test_resize_thumb() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_resized.jpg");
    let _ = Client::new(key)
      .from_file("./tmp_image.jpg")
      .await?
      .resize(Resize::new(Method::THUMB, Some(400), Some(200)))
      .await?
      .to_file(output);

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

  #[tokio::test]
  async fn test_error_transparent_png_to_jpeg() -> Result<(), TinifyError> {
    let key = get_key();
    let request = Client::new(key)
      .from_url("https://tinypng.com/images/panda-happy.png")
      .await?
      .convert((Some(Type::JPEG), None, None), None)
      .await
      .unwrap_err();

    assert_matches!(request, TinifyError::ClientError { .. });

    Ok(())
  }

  #[tokio::test]
  async fn test_transparent_png_to_jpeg() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./panda-sticker.jpg");
    let _ = Client::new(key)
      .from_url("https://tinypng.com/images/panda-happy.png")
      .await?
      .convert((Some(Type::JPEG), None, None), Some(Color("#000000")))
      .await?
      .to_file(output);

    let extension = output.extension().and_then(OsStr::to_str).unwrap();

    assert_eq!(extension, "jpg");

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_convert_from_jpg_to_png() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./tmp_converted.png");
    let _ = Client::new(key)
      .from_file(Path::new("./tmp_image.jpg"))
      .await?
      .convert((Some(Type::PNG), None, None), None)
      .await?
      .to_file(output);

    let extension = output.extension().and_then(OsStr::to_str).unwrap();

    assert_eq!(extension, "png");

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_convert_from_jpg_to_webp() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./panda-sticker.webp");
    let _ = Client::new(key)
      .from_url("https://tinypng.com/images/panda-happy.png")
      .await?
      .convert((Some(Type::WEBP), None, None), None)
      .await?
      .to_file(output);

    let extension = output.extension().and_then(OsStr::to_str).unwrap();

    assert_eq!(extension, "webp");

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_convert_smallest_type() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./panda-sticker.webp");
    let _ = Client::new(key)
      .from_url("https://tinypng.com/images/panda-happy.png")
      .await?
      .convert((Some(Type::PNG), Some(Type::WEBP), Some(Type::JPEG)), None)
      .await?
      .to_file(output);

    let extension = output.extension().and_then(OsStr::to_str).unwrap();

    assert_eq!(extension, "webp");

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_convert_smallest_wildcard_type() -> Result<(), TinifyError> {
    let key = get_key();
    let output = Path::new("./panda-sticker.webp");
    let _ = Client::new(key)
      .from_url("https://tinypng.com/images/panda-happy.png")
      .await?
      .convert((Some(Type::WILDCARD), None, None), None)
      .await?
      .to_file(output);

    let extension = output.extension().and_then(OsStr::to_str).unwrap();

    assert_eq!(extension, "webp");

    if output.exists() {
      fs::remove_file(output)?;
    }

    Ok(())
  }
}
