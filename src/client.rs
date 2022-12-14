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
