use crate::error::{self, TinifyException};
use reqwest::blocking::Client as BlockingClient;
use reqwest::blocking::Response as ReqwestResponse;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::process;
use std::str;

type TinifyError = ReqwestError;
type TinifyResponse = ReqwestResponse;

const API_ENDPOINT: &str = "https://api.tinify.com";

pub enum Method {
  Post,
  Get,
}

#[derive(Debug, PartialEq)]
pub struct Source {
  url: Option<String>,
  key: Option<String>,
  buffer: Option<Vec<u8>>,
}

impl Source {
  pub fn new(
    url: Option<String>,
    key: Option<String>,
  ) -> Self {
    Self {
      url,
      key,
      buffer: None,
    }
  }

  pub fn request(
    &self,
    method: Method,
    url: &str,
    buffer: Option<&[u8]>,
  ) -> Result<TinifyResponse, TinifyError> {
    let parse = format!("{}{}", API_ENDPOINT, url);
    let reqwest_client = BlockingClient::new();
    let timeout = Duration::from_secs(240);
    let resp = match method {
      Method::Post => {
        let resp= reqwest_client
          .post(parse)
          .body(buffer.unwrap().to_owned())
          .basic_auth("api", self.key.as_ref())
          .timeout(timeout)
          .send();

        resp
      },
      Method::Get => {
        let resp = reqwest_client
          .get(url)
          .timeout(timeout)
          .send();

        resp
      },
    };
    if let Err(error) = resp.as_ref() {
      if error.is_connect() {
        eprintln!("Error processing the request.");
        process::exit(1);
      }
    }
    let request_status = resp.as_ref().unwrap().status();

    match request_status {
      StatusCode::UNAUTHORIZED => {
        error::exit_error(
          TinifyException::AccountException, 
          &request_status
        );
      },
      StatusCode::UNSUPPORTED_MEDIA_TYPE => {
        error::exit_error(
          TinifyException::ClientException, 
          &request_status
        );
      },
      StatusCode::SERVICE_UNAVAILABLE => {
        error::exit_error(
          TinifyException::ServerException, 
          &request_status
        );
      },
      _  => {},
    };
    
    resp
  }

  pub fn from_file(
    self,
    path: &Path,
  ) -> Result<Self, TinifyException> {
    let location = Path::new(path);
    if !location.exists() {
      return Err(TinifyException::NoFileOrDirectory);
    }
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::with_capacity(reader.capacity());
    reader.read_to_end(&mut buffer).unwrap();
    
    Ok(self.from_buffer(&buffer))
  }

  pub fn from_buffer(self, buffer: &[u8]) -> Self {
    let resp = self
      .request(Method::Post, "/shrink", Some(buffer));

    self.get_source_from_response(resp.unwrap())
  }

  pub fn from_url(
    self,
    url: &str,
  ) -> Result<Self, TinifyException> {
    let get_resp =
      self.request(Method::Get, url, None);
    let bytes =
      get_resp.unwrap().bytes().unwrap().to_vec();
    let post_resp = self
      .request(Method::Post, "/shrink", Some(&bytes));

    Ok(self.get_source_from_response(post_resp.unwrap()))
  }

  pub fn get_source_from_response(
    mut self,
    response: TinifyResponse,
  ) -> Self {
    let optimized_location = response
      .headers()
      .get("location")
      .unwrap();
    
    let mut url = String::new();
    if !optimized_location.is_empty() {
      let slice =
        str::from_utf8(optimized_location.as_bytes()).unwrap();
      url.push_str(slice);
    }
    let bytes = self.request(Method::Get, &url, None);
    let compressed =
      bytes.unwrap().bytes().unwrap().to_vec();
    self.buffer = Some(compressed);
    self.url = Some(url);

    self
  }

  pub fn to_file(&self, path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut reader = BufWriter::new(file);
    reader.write_all(self.buffer.as_ref().unwrap())?;
    reader.flush()?;

    Ok(())
  }

  pub fn to_buffer(&self) -> Vec<u8> {
    self.buffer.as_ref().unwrap().to_vec()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dotenv::dotenv;
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
  fn test_get_request() -> Result<(), TinifyError> {
    let source = Source::new(None, None);
    let url = "https://tinypng.com/images/panda-happy.png";
    let _ = source.request(Method::Get, url, None)?;

    Ok(())
  }
  
  #[test]
  fn test_post_request() -> Result<(), TinifyError> {
    dotenv().ok();
    let key = get_key();
    let source = Source::new(None, Some(key));
    let path = Path::new("./tmp_image.jpg");
    let bytes = fs::read(path).unwrap();
    let _ = source
      .request(Method::Post, "/shrink", Some(&bytes))?;

    Ok(())
  }

  #[test]
  fn test_from_file() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let path = Path::new("./tmp_image.jpg");
    let source = Source::new(None, Some(key));
    let _ = source.from_file(path)?;

    Ok(())
  }

  #[test]
  fn test_from_url() -> Result<(), TinifyException> {
    dotenv().ok();
    let key = get_key();
    let url = "https://tinypng.com/images/panda-happy.png";
    let _ = Source::new(None, Some(key)).from_url(url)?;

    Ok(())
  }

  #[test]
  fn test_get_source_from_response() {
    let key = get_key();
    let path = Path::new("./tmp_image.jpg");
    let source = Source::new(None, Some(key.clone()));
    let bytes = fs::read(path).unwrap();
    let get_resp = source.request(Method::Post, "/shrink", Some(&bytes)).unwrap();
    let actual = source.get_source_from_response(get_resp);
    let mut expected = Source::new(None, Some(key.clone()));
    expected.buffer = actual.buffer.clone();
    expected.url = actual.url.clone();

    assert_eq!(actual, expected);
  }

  #[test]
  fn test_to_file() {
    let key = get_key();
    let tmp = "./tmp_image.jpg";
    let location = "./new_image.jpg";
    let bytes = fs::read(tmp).unwrap();
    let mut source = Source::new(None, Some(key.clone()));
    source.buffer = Some(bytes);
    let _ = source.to_file(location);
    let exists = Path::exists(Path::new(location));

    assert!(exists);

    if exists {
      fs::remove_file(location).unwrap();
    }
  }

  #[test]
  fn test_to_buffer() {
    let tmp = "./tmp_image.jpg";
    let expected = fs::read(tmp).unwrap();
    let mut source = Source::new(None, None);
    source.buffer = Some(expected.clone());
    let actual = source.to_buffer();

    assert_eq!(actual, expected);
  }
}
