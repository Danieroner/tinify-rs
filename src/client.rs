use crate::error;
use crate::error::{TinifyException, TinifyResult};
use reqwest::blocking::Client as BlockingClient;
use reqwest::StatusCode;
use std::time::Duration;
use std::path::Path;
use std::process;

const API_ENDPOINT: &str = "https://api.tinify.com";

pub enum Method {
  POST,
  GET,
}

#[derive(Debug, PartialEq)]
pub struct Client {
  pub key: String,
}

impl Client {
  pub fn new(key: String) -> Self {
    Self { key }
  }

  pub fn request(
    &self, 
    method: Method, 
    path: &Path, 
    buffer: Option<&Vec<u8>>
  ) -> TinifyResult {
    let url = format!(
      "{}{}",
      API_ENDPOINT,
      path.to_str().unwrap(),
    );
    let client = BlockingClient::new();
    let timeout = Duration::from_secs(240 as u64);
    let response = match method {
      Method::POST => {
        let response = client
          .post(url)
          .body(buffer.unwrap().to_vec())
          .basic_auth("api", Some(&self.key))
          .timeout(timeout)
          .send();

        response
      },
      Method::GET => {
        let response = client.get(path.to_str().unwrap())
          .timeout(timeout)
          .send();

        response
      },
    };

    if let Err(error) = &response {
      if error.is_connect() {
        eprintln!("Error processing the request.");
        process::exit(1);
      }
    }
    
    let request_status = response
      .as_ref()
      .unwrap()
      .status();

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
      _  => {()},
    };
    
    response
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
