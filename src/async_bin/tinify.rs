use crate::async_bin::client::Client;
use crate::error::TinifyError;

/// Use the API to create a new client.
#[derive(Default)]
pub struct Tinify {
  pub key: String,
}

impl Tinify {
  /// Create a new Tinify Object.
  pub fn new() -> Self {
    Self { key: String::new() }
  }

  /// Set a Tinify Key.
  pub fn set_key<K>(mut self, key: K) -> Self
  where
    K: Into<String>,
  {
    self.key = key.into();
    self
  }

  /// Get a new Tinify Client.
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
  ///
  ///   Ok(())
  /// }
  /// ```
  pub fn get_async_client(&self) -> Result<Client, TinifyError> {
    let client = Client::new(&self.key);

    Ok(client)
  }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod tests {
  use super::*;
  use dotenv::dotenv;
  use std::env;

  #[test]
  fn test_get_async_client() -> Result<(), TinifyError> {
    dotenv().ok();
    let key = match env::var("KEY") {
      Ok(key) => key,
      Err(_err) => panic!("No such file or directory."),
    };
    let _ = Tinify::new().set_key(key).get_async_client()?;

    Ok(())
  }
}
