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
    &self,
    response:
    TinifyResponse
  ) -> Self {
    let location = response
      .headers()
      .get("location")
      .unwrap();

    let mut url = String::new();

    if location.len() > 0 {
      url.push_str(
        str::from_utf8(&location.as_bytes()).unwrap()
      );
    }

    let bytes = tinify::get_client()
      .request(
        Method::GET,
        Path::new(&url),
        None,
      );

    let compressed_buffer = bytes
      .unwrap()
      .bytes()
      .unwrap()
      .to_vec();

    let mut buffer_state = BUFFER
      .lock()
      .expect("Could not lock mutex");

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
  use crate::mock::MockClient;

  lazy_static! {
    static ref MOCK_CLIENT: MockClient = MockClient::new();
    static ref TMP_PATH: &'static str = "./tmp_image.jpg";
  }

  #[test]
  fn test_from_file_get_source() {
    tinify::set_key(MOCK_CLIENT.key.as_str());
    let source = Source::new(None).from_file(*TMP_PATH);
    let expected = Source::new(source.url.clone());
    
    assert_eq!(source, expected);
  }

  #[test]
  fn test_from_buffer_get_source() {
    tinify::set_key(MOCK_CLIENT.key.as_str());
    let path = Path::new(*TMP_PATH);
    let buffer = fs::read(path).unwrap();
    let source = Source::new(None).from_buffer(buffer);
    let expected = Source::new(source.url.clone());

    assert_eq!(source, expected);
  }

  #[test]
  fn test_from_url_get_source() {
    tinify::set_key(MOCK_CLIENT.key.as_str());
    let path = "https://tinypng.com/images/panda-happy.png";
    let source = Source::new(None).from_url(path);
    let expected = Source::new(source.url.clone());

    assert_eq!(source, expected);
  }

  #[test]
  fn test_get_source_from_response() {
    let buffer = fs::read(*TMP_PATH).unwrap();
    let url_endpoint = Path::new("/shrink");
    tinify::set_key(MOCK_CLIENT.key.as_str());
    let response = MOCK_CLIENT.request(
      Method::POST, 
      url_endpoint, 
      Some(&buffer),
    );
    let source = Source::new(None)
      .get_source_from_response(response.unwrap());
    let expected = Source::new(source.url.clone());
    
    assert_eq!(source, expected);
  }
}
