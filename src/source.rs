use crate::result;
use crate::tinify;
use crate::error::TinifyResponse;
use crate::client::Method;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::Path;
use std::process;
use std::mem;
use std::str;
use std::io;
use std::fs;

lazy_static! {
  static ref BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}

#[derive(Debug, PartialEq)]
pub struct Source {
  pub url: Option<String>,
}

#[allow(unused_must_use)]
impl Source {
  pub fn new(url: Option<String>) -> Self {
    Self { url }
  }

  fn replace_buffer(&self, buffer: &mut Vec<u8>, compressed_image: Vec<u8>) {
    mem::replace(&mut *buffer, compressed_image);
  }

  pub fn from_file(&mut self, path: &str) -> Self {
    let route = Path::new(path);
    if !route.exists() {
      eprintln!("No such file or directory.");
      process::exit(1);
    }
    let buffer = fs::read(route).unwrap();
    
    self.from_buffer(buffer)
  }

  pub fn from_buffer(&self, buffer: Vec<u8>) -> Self {
    let response = tinify::get_client()
      .request(
        Method::POST,
        Path::new("/shrink"),
        Some(&buffer),
    );

    self.get_source_from_response(response.unwrap())
  }

  pub fn get_source_from_response(&self, response: TinifyResponse) -> Self {
    let location = response.headers().get("location").unwrap();
    let mut url = String::new();
    if location.len() > 0 {
      url.push_str(str::from_utf8(&location.as_bytes()).unwrap());
    }
    let bytes = tinify::get_client()
      .request(
        Method::GET,
        Path::new(&url),
        None,
    );
    let compressed_buffer = bytes.unwrap().bytes().unwrap().to_vec();
    let mut buffer_state = BUFFER.lock().expect("Could not lock mutex");
    self.replace_buffer(&mut buffer_state, compressed_buffer);
    let source = Source::new(Some(url));

    source
  }

  pub fn result(&self) -> result::Result {
    if self.url.as_ref().unwrap().len() == 0 {
      eprintln!("Url is empty.");
      process::exit(1);
    }
    let result = result::Result {
      data: BUFFER.lock().unwrap(),
    };

    result
  }

  pub fn to_file(&self, path: &str) -> io::Result<()> {
    self.result().to_file(&path, self.url.as_ref())
  }

  pub fn to_buffer(&self) -> Vec<u8> {
    self.result().to_buffer()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::create_file;
  use crate::tmp_file::MockClient;

  lazy_static! {
    static ref TMP_PATH: &'static str = "./tmp_test_image.png";
  }

  #[test]
  fn test_from_file_get_source() {
    let path = Path::new(*TMP_PATH);
    if !path.exists() {
      create_file!();
    }
    let mock_client = MockClient::new();
    tinify::set_key(mock_client.key);
    let source = Source::new(None).from_file(*TMP_PATH);
    let expected = Source::new(source.url.clone());
    if path.exists() {
      fs::remove_file(path).unwrap();
    }
    
    assert_eq!(source, expected);
  }

  #[test]
  fn test_from_buffer_get_source() {
    let mock_client = MockClient::new();
    tinify::set_key(mock_client.key);
    let path = Path::new(*TMP_PATH);
    if !path.exists() {
      create_file!();
    }
    let buffer = fs::read(path).unwrap();
    let source = Source::new(None).from_buffer(buffer);
    let expected = Source::new(source.url.clone());
    if path.exists() {
      fs::remove_file(path).unwrap();
    }

    assert_eq!(source, expected);
  }

  #[test]
  fn test_get_source_from_response() {
    let path = Path::new(*TMP_PATH);
    if !path.exists() {
      create_file!();
    }
    let buffer = fs::read(*TMP_PATH).unwrap();
    let url_endpoint = Path::new("/shrink");
    let mock_client = MockClient::new();
    tinify::set_key(mock_client.key);
    let response = mock_client.request(
      Method::POST, 
      url_endpoint, 
      Some(&buffer),
    );
    let source = Source::new(None)
      .get_source_from_response(response.unwrap());
    let expected = Source::new(source.url.clone());
    if path.exists() {
      fs::remove_file(path).unwrap();
    }
    
    assert_eq!(source, expected);
  }
}
