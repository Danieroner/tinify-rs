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

#[derive(Debug, PartialEq, Eq)]
pub struct Source {
  url: Option<String>,
  key: Option<String>,
  buffer: Option<Vec<u8>>,
}

impl Source {
  pub fn new(url: Option<String>, key: Option<String>) -> Self {
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
    let full_url = format!("{}{}", API_ENDPOINT, url);
    let reqwest_client = BlockingClient::new();
    let timeout = Duration::from_secs(240);
    let response = match method {
      Method::Post => {
        reqwest_client
          .post(full_url)
          .body(buffer.unwrap().to_owned())
          .basic_auth("api", self.key.as_ref())
          .timeout(timeout)
          .send()
      },
      Method::Get => {
        reqwest_client
          .get(url)
          .timeout(timeout)
          .send()
      },
    };
    if let Err(error) = response.as_ref() {
      if error.is_connect() {
        eprintln!("Error processing the request.");
        process::exit(1);
      }
    }
    let request_status = response.as_ref().unwrap().status();

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
    
    response
  }

  pub fn from_file(self, path: &Path) -> Result<Self, TinifyException> {
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
    let response =
      self.request(Method::Post, "/shrink", Some(buffer));

    self.get_source_from_response(response.unwrap())
  }

  pub fn from_url(self, url: &str) -> Result<Self, TinifyException> {
    let get = self.request(Method::Get, url, None);
    let bytes = get.unwrap().bytes().unwrap().to_vec();
    let post = self.request(Method::Post, "/shrink", Some(&bytes));

    Ok(self.get_source_from_response(post.unwrap()))
  }

  pub fn get_source_from_response(
    mut self,
    response: TinifyResponse,
  ) -> Self {
    let optimized_location =
      response.headers().get("location").unwrap();
    let mut url = String::new();

    if !optimized_location.is_empty() {
      let slice =
        str::from_utf8(optimized_location.as_bytes()).unwrap();
      url.push_str(slice);
    }
    
    let get = self.request(Method::Get, &url, None);
    let bytes = get.unwrap().bytes().unwrap().to_vec();
    self.buffer = Some(bytes);
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
