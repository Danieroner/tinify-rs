#![allow(dead_code)]

use crate::client::{Client, Method};
use crate::error::TinifyResult;
use dotenv::dotenv;
use std::env;
use std::path::Path;

pub struct MockClient {
  pub key: String,
}

impl MockClient {
  pub fn new() -> Self {
    dotenv().ok();
    let var = match env::var("KEY") {
      Ok(val) => val,
      Err(_err) => "none".to_string(),
    };
    Self { key: var }
  }

  pub fn request(
    &self,
    method: Method,
    path: &Path,
    buffer: Option<&Vec<u8>>
) -> TinifyResult {
    let client = Client {
      key: self.key.to_owned(),
    };
    client.request(method, path, buffer)
  }
}
