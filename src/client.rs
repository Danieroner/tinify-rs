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
  use crate::mock::MockClient;
  use lazy_static::lazy_static;
  use std::path::Path;
  use std::fs;

  lazy_static! {
    static ref MOCK_CLIENT: MockClient = MockClient::new();
  }

  #[test]
  fn test_get_200_status_request() {
    let test_url = Path::new(API_ENDPOINT);
    let response = MOCK_CLIENT
      .request(Method::GET, test_url, None)
      .unwrap();
  
    assert_eq!(response.status(), 200);
  }

  #[test]
  fn test_post_201_status_created() {
    let tmp_image = fs::read("./tmp_image.jpg").unwrap();
    let buffer = tmp_image.to_vec();
    let url_endpoint = Path::new("/shrink");
    let response = MOCK_CLIENT.request(
      Method::POST, 
      url_endpoint, 
      Some(&buffer),
    );
    
    assert_eq!(response.unwrap().status(), 201);
  }
}
