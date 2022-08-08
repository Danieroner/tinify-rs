use crate::TinifyException;
use crate::source::Source;
use std::path::Path;

#[derive(Debug)]
pub struct Client {
  pub key: String,
}

impl Client {
  pub fn new(key: String) -> Self {
    Self { key }
  }
  
  fn get_source(&self) -> Source {
    Source::new(None, Some(self.key.clone())) 
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
  pub fn from_file(
    &self,
    path: &str,
  ) -> Result<Source, TinifyException> {
    let path = Path::new(path);
    let source = self
      .get_source()
      .from_file(path);
  
    source
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
  ) -> Result<Source, TinifyException> {
    let source = self
      .get_source()
      .from_buffer(buffer);
  
    Ok(source)
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
  pub fn from_url(
    &self,
    url: &str,
  ) -> Result<Source, TinifyException> {
    let source = Source::new(None, Some(self.key.clone()))
      .from_url(url);
  
    source
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Tinify;
  use crate::source::Source;
  use reqwest::blocking::Client as BlockingClient; 
  use dotenv::dotenv;
  use std::default::Default;
  use std::env;
  use std::fs;

  fn get_key() -> String {
    let key = match env::var("KEY") {
      Ok(key) => key,
      Err(_err) => panic!("No such file or directory."),
    };
  
    key
  }

  #[test]
  fn test_get_source() {
    let actual = Client::new(String::new())
      .get_source();
    let expected =
      Source::new(Default::default(), Some(String::new()));

    assert_eq!(actual, expected);
  }

  #[test]
  fn test_from_file() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let _ = Client::new(key).from_file("./tmp_image.jpg")?;
  
    Ok(())
  }

  #[test]
  fn test_from_buffer() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let buffer = fs::read("./tmp_image.jpg").unwrap();
    let _ = Client::new(key).from_buffer(&buffer)?;
  
    Ok(())
  }

  #[test]
  fn test_from_url() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let url = "https://tinypng.com/images/panda-happy.png";
    let _ = Client::new(key).from_url(url)?;
  
    Ok(())
  }

  #[test]
  fn test_compressed_from_file_to_file() -> Result<(), TinifyException>  {
    dotenv().ok();
    let key = get_key();
    let client = Tinify::new()
      .set_key(&key)
      .get_client()?;

    let _ = client
      .from_file("./tmp_image.jpg")?
      .to_file("./tmp_compressed.jpg")
      .unwrap();
    
    let uncompress_size =
      fs::metadata("./tmp_image.jpg").unwrap().len();

    let path = Path::new("./tmp_compressed.jpg");
    let compress_size =
      fs::metadata("./tmp_compressed.jpg").unwrap().len();
    let exists = Path::exists(path);

    assert_eq!(uncompress_size, 124814);
    assert_eq!(compress_size, 102051);

    if exists {
      fs::remove_file(path).unwrap();
    }

    Ok(())
  }

  #[test]
  fn test_compressed_from_file_to_buffer() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let client = Tinify::new()
      .set_key(&key)
      .get_client()?;

    let buffer = client
      .from_file("./tmp_image.jpg")?
      .to_buffer();

    let path = Path::new("./tmp_compressed.jpg");
    fs::write(path, &buffer).unwrap();

    let expected = fs::read(path).unwrap();
    let exists = Path::exists(path);

    assert_eq!(buffer, expected);

    if exists {
      fs::remove_file(path).unwrap();
    }

    Ok(())
  }

  #[test]
  fn test_compressed_from_buffer_to_file() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let client = Tinify::new()
      .set_key(&key)
      .get_client()?;

    let path = Path::new("./tmp_image.jpg");
    let uncompress = fs::read(path).unwrap();
    let uncompress_size = fs::metadata(path).unwrap().len();

    let _ = client
      .from_buffer(&uncompress)?
      .to_file("./tmp_compressed.jpg");

    let compress_size = fs::metadata("./tmp_compressed.jpg").unwrap().len();

    assert_eq!(uncompress_size, 124814);
    assert_eq!(compress_size, 102051);

    let path = Path::new("./tmp_compressed.jpg");
    let exists = Path::exists(path);

    if exists {
      fs::remove_file(path).unwrap();
    }

    Ok(())
  }

  #[test]
  fn test_compressed_from_buffer_to_buffer() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let client = Tinify::new()
      .set_key(&key)
      .get_client()?;

    let path = Path::new("./tmp_image.jpg");
    let uncompress = fs::read(path).unwrap();

    let buffer = client
      .from_buffer(&uncompress)?
      .to_buffer();

    fs::write("./tpm_compressed.jpg", &buffer).unwrap();
    let expected = fs::read("./tpm_compressed.jpg").unwrap();

    assert_eq!(buffer, expected);

    let path = Path::new("./tmp_compressed.jpg");
    let exists = Path::exists(path);

    if exists {
      fs::remove_file(path).unwrap();
    }


    Ok(())
  }

  #[test]
  fn test_compressed_from_url_to_file() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let client = Tinify::new()
      .set_key(&key)
      .get_client()?;

    let uncompress_size = BlockingClient::new()
      .get("https://tinypng.com/images/panda-happy.png")
      .send()
      .unwrap()
      .content_length()
      .unwrap();

    let _ = client
      .from_url("https://tinypng.com/images/panda-happy.png")?
      .to_file("./tmp_compressed.jpg");
   
    let compress_size =
      fs::metadata("./tmp_compressed.jpg").unwrap().len();
  
    assert_eq!(uncompress_size, 30734);
    assert_eq!(compress_size, 26324);

    let path = Path::new("./tmp_compressed.jpg");
    let exists = Path::exists(path);

    if exists {
      fs::remove_file(path).unwrap();
    }

    Ok(())
  }

  #[test]
  fn test_compressed_from_url_to_buffer() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let client = Tinify::new()
      .set_key(&key)
      .get_client()?;
    
    let buffer = client
      .from_url("https://tinypng.com/images/panda-happy.png")?
      .to_buffer();

    fs::write("./tpm_compressed.jpg", &buffer).unwrap();
    let expected = fs::read("./tmp_compressed.jpg").unwrap();

    assert_eq!(buffer, expected);

    let path = Path::new("./tmp_compressed.jpg");
    let exists = Path::exists(path);

    if exists {
      fs::remove_file(path).unwrap();
    }

    Ok(())
  }
}
