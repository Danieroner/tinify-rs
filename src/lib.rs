//! # Tinify Crate
//!
//! `tinify-rs` is a Rust client for the Tinify API.
//! Used for TinyPNG and TinyJPG. Tinify compresses your images intelligently.
//!
//! Read more at `https://tinify.com`
// --snip--

mod error;
mod client;
mod source;
mod resize;
pub mod convert;

pub use crate::client::Client;
pub use crate::source::Source;
pub use crate::resize::Resize;
pub use crate::resize::ResizeMethod;
pub use crate::error::TinifyError;

/// Use the API to create a new client.
#[derive(Default)]
pub struct Tinify {
  pub key: String,
}

impl Tinify {
  /// Create a new Tinify Object.
  pub fn new() -> Self {
    Self {
      key: String::new(),
    }
  }
  
  /// Set a Tinify Key.
  pub fn set_key<K>(
    mut self,
    key: K,
  ) -> Self
  where
    K: AsRef<str> + Into<String>,
  {
    self.key = key.into();
    self
  }

  /// Get a new Tinify Client.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// use tinify::TinifyError;
  /// 
  /// fn main() -> Result<(), TinifyError> {
  ///   let key = "tinify api key";
  ///   let tinify = Tinify::new().set_key(key);
  ///   let client = tinify.get_client()?;
  /// 
  ///   Ok(())
  /// }
  /// ```
  pub fn get_client(&self) -> Result<Client, TinifyError> {
    let client = Client::new(self.key.as_str());
  
    Ok(client)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenv::dotenv;
  use std::env;

  #[test]
  fn test_get_client() -> Result<(), TinifyError> {
    dotenv().ok();
    let key = match env::var("KEY") {
      Ok(key) => key,
      Err(_err) => panic!("No such file or directory."),
    };
    let _ = Tinify::new()
      .set_key(&key)
      .get_client()?;

    Ok(())
  }
}
