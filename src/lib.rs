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

pub use crate::client::Client;
pub use crate::source::Source;
pub use crate::error::TinifyError;

#[derive(Debug)]
pub struct Tinify {
  pub key: String,
}

impl Tinify {
  /// Create a new Tinify Object.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// 
  /// fn main() {
  ///   let tinify = Tinify::new();
  /// }
  /// ```
  pub fn new() -> Self {
    Self {
      key: String::new(),
    }
  }
  
  /// Set a Tinify Key.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// 
  /// fn main() {
  ///   let key = "tinify api key";
  ///   let tinify = Tinify::new().set_key(key);
  /// }
  /// ```
  pub fn set_key(mut self, key: &str) -> Self {
    self.key = key.to_string();
    self
  }

  /// Get a new Tinify Client.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::{Tinify, TinifyException};
  /// 
  /// fn main() -> Result<(), TinifyException> {
  ///   let key = "tinify api key";
  ///   let tinify = Tinify::new().set_key(key);
  ///   let client = tinify.get_client()?;
  ///   
  ///   Ok(())
  /// }
  /// ```
  pub fn get_client(&self) -> Result<Client, TinifyError> {
    let client = Client::new(self.key.to_string());
  
    Ok(client)
  }
}
