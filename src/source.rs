use crate::error::TinifyError;
use crate::resize::JsonData;
use crate::resize::Resize;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::Response as ReqwestResponse;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use reqwest::Method;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::str;

type TinifyResponse = ReqwestResponse;

const API_ENDPOINT: &str = "https://api.tinify.com";

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
    let full_url =
      format!("{}{}", API_ENDPOINT, url);
    let reqwest_client = ReqwestClient::new();
    let response = match method {
      Method::POST => {
        reqwest_client
          .post(full_url)
          .body(buffer.unwrap().to_owned())
          .basic_auth("api", self.key.as_ref())
          .timeout(Duration::from_secs(300))
          .send()?
      },
      Method::GET => {
        reqwest_client
          .get(url)
          .timeout(Duration::from_secs(300))
          .send()?
      },
      _ => unreachable!(),
    };
    
    match response.status() {
      StatusCode::UNAUTHORIZED => {
        return Err(TinifyError::ClientError);
      },
      StatusCode::UNSUPPORTED_MEDIA_TYPE => {
        return Err(TinifyError::ClientError);
      },
      StatusCode::SERVICE_UNAVAILABLE => {
        return Err(TinifyError::ServerError);
      },
      _  => {},
    };

    Ok(response)
  }

  pub fn from_file(self, path: &Path) -> Result<Self, TinifyError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::with_capacity(reader.capacity());
    reader.read_to_end(&mut buffer).unwrap();
    
    Ok(self.from_buffer(&buffer))
  }

  pub fn from_buffer(self, buffer: &[u8]) -> Self {
    let response =
      self.request(Method::POST, "/shrink", Some(buffer));

    self.get_source_from_response(response.unwrap())
  }

  pub fn from_url(self, url: &str) -> Result<Self, TinifyError> {
    let get = self.request(Method::GET, url, None);
    let bytes = get.unwrap().bytes().unwrap().to_vec();
    let post = self.request(Method::POST, "/shrink", Some(&bytes));

    Ok(self.get_source_from_response(post.unwrap()))
  }

  /// Resize the current compressed image.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// use tinify::Client;
  /// use tinify::TinifyError;
  /// use tinify::ResizeMethod;
  /// use tinify::Resize;
  /// 
  /// fn get_client() -> Result<Client, TinifyError> {
  ///   let key = "tinify api key";
  ///   let tinify = Tinify::new();
  ///
  ///   tinify
  ///     .set_key(key)
  ///     .get_client()
  /// }
  /// 
  /// fn main() -> Result<(), TinifyError> {
  ///  let client = get_client()?;
  ///  let _ = client
  ///    .from_file("./unoptimized.jpg")?
  ///    .resize(Resize::new(
  ///      ResizeMethod::FIT,
  ///      Some(400),
  ///      Some(200)),
  ///    )?
  ///    .to_file("./resized.jpg")?;
  ///
  ///  Ok(())
  /// }
  /// ```
  pub fn resize(
    self,
    resize: Resize,
  ) -> Result<Self, TinifyError> {
    let json_data = JsonData::new(resize);
    let mut json_string =
      serde_json::to_string(&json_data).unwrap();
    let width = json_data.resize.width;
    let height = json_data.resize.height;
    json_string = match (
      (width.is_some(), height.is_none()),
      (height.is_some(), width.is_none()),
    ) {
      ((true, true), (_, _)) =>
        json_string.replace(",\"height\":null", ""),
      ((_, _), (true, true)) =>
        json_string.replace(",\"width\":null", ""),
      _ => json_string,
    };
    let reqwest_client = ReqwestClient::new();
    let response = reqwest_client
      .post(self.url.as_ref().unwrap())
      .header(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
      )
      .body(json_string)
      .basic_auth("api", self.key.as_ref())
      .timeout(Duration::from_secs(300))
      .send()?;

    if response.status() == StatusCode::BAD_REQUEST {
      return Err(TinifyError::ClientError);
    }
    
    Ok(self.get_source_from_response(response))
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
    
    let get = self.request(Method::GET, &url, None);
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
