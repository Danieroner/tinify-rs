use crate::convert::Color;
use crate::convert::Convert;
use crate::convert::JsonData;
use crate::convert::Transform;
use crate::error::TinifyError;
use crate::resize;
use crate::API_ENDPOINT;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::Response;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use reqwest::StatusCode;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str;
use std::time::Duration;

#[derive(Debug)]
pub struct Source {
  url: Option<String>,
  key: Option<String>,
  buffer: Option<Vec<u8>>,
  request_client: ReqwestClient,
}

impl Source {
  pub(crate) fn new(url: Option<&str>, key: Option<&str>) -> Self {
    let url = url.map(|val| val.into());
    let key = key.map(|val| val.into());
    let request_client = ReqwestClient::new();

    Self {
      url,
      key,
      buffer: None,
      request_client,
    }
  }

  pub(crate) fn request<U>(
    &self,
    url: U,
    method: Method,
    buffer: Option<&[u8]>,
  ) -> Result<Response, TinifyError>
  where
    U: AsRef<str>,
  {
    let full_url = format!("{}{}", API_ENDPOINT, url.as_ref());
    let response = match method {
      Method::POST => self
        .request_client
        .post(full_url)
        .body(buffer.unwrap().to_vec())
        .basic_auth("api", self.key.as_ref())
        .timeout(Duration::from_secs(300))
        .send()?,
      Method::GET => self
        .request_client
        .get(url.as_ref())
        .timeout(Duration::from_secs(300))
        .send()?,
      _ => unreachable!(),
    };

    match response.status() {
      StatusCode::UNAUTHORIZED => Err(TinifyError::ClientError),
      StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(TinifyError::ClientError),
      StatusCode::SERVICE_UNAVAILABLE => Err(TinifyError::ServerError),
      _ => Ok(response),
    }
  }

  #[allow(clippy::wrong_self_convention)]
  pub(crate) fn from_file<P>(self, path: P) -> Result<Self, TinifyError>
  where
    P: AsRef<Path>,
  {
    let file =
      File::open(path).map_err(|source| TinifyError::ReadError { source })?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(reader.capacity());
    reader.read_to_end(&mut buffer)?;

    self.from_buffer(&buffer)
  }

  #[allow(clippy::wrong_self_convention)]
  pub(crate) fn from_buffer(self, buffer: &[u8]) -> Result<Self, TinifyError> {
    let response = self.request("/shrink", Method::POST, Some(buffer))?;

    self.get_source_from_response(response)
  }

  #[allow(clippy::wrong_self_convention)]
  pub(crate) fn from_url<U>(self, url: U) -> Result<Self, TinifyError>
  where
    U: AsRef<str>,
  {
    let get_request = self.request(url, Method::GET, None);
    let buffer = get_request?.bytes()?;
    let post_request = self.request("/shrink", Method::POST, Some(&buffer))?;

    self.get_source_from_response(post_request)
  }

  /// Resize the current compressed image.
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::sync::Tinify;
  /// use tinify::sync::Client;
  /// use tinify::error::TinifyError;
  /// use tinify::resize::Method;
  /// use tinify::resize::Resize;
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
  ///   let client = get_client()?;
  ///   let _ = client
  ///     .from_file("./unoptimized.jpg")?
  ///     .resize(Resize::new(
  ///       Method::FIT,
  ///       Some(400),
  ///       Some(200),
  ///     ))?
  ///     .to_file("./resized.jpg")?;
  ///
  ///   Ok(())
  /// }
  /// ```
  pub fn resize(self, resize: resize::Resize) -> Result<Self, TinifyError> {
    let json_data = resize::JsonData::new(resize);
    let mut json_string = serde_json::to_string(&json_data).unwrap();
    let width = json_data.resize.width;
    let height = json_data.resize.height;
    json_string = match (
      (width.is_some(), height.is_none()),
      (height.is_some(), width.is_none()),
    ) {
      ((true, true), (_, _)) => json_string.replace(",\"height\":null", ""),
      ((_, _), (true, true)) => json_string.replace(",\"width\":null", ""),
      _ => json_string,
    };
    let response = self
      .request_client
      .post(self.url.as_ref().unwrap())
      .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
      .body(json_string)
      .basic_auth("api", self.key.as_ref())
      .timeout(Duration::from_secs(300))
      .send()?;

    match response.status() {
      StatusCode::BAD_REQUEST => Err(TinifyError::ClientError),
      _ => self.get_source_from_response(response),
    }
  }

  /// The following options are available as a type:
  /// One image type, specified as a string `"image/webp"`
  ///
  /// Multiple image types, specified as a tuple (`"image/webp"`, `"image/png"`).
  /// The smallest of the provided image types will be returned.
  ///
  /// The transform object specifies the stylistic transformations
  /// that will be applied to the image.
  ///
  /// Include a background property to fill a transparent image's background.
  ///
  /// Specify a background color to convert an image with a transparent background
  /// to an image type which does not support transparency (like JPEG).
  ///
  /// # Examples
  ///
  /// ```
  /// use tinify::Tinify;
  /// use tinify::convert::Color;
  /// use tinify::convert::Type;
  /// use tinify::TinifyError;
  ///
  /// fn main() -> Result<(), TinifyError> {
  ///   let _ = Tinify::new()
  ///     .set_key("api key")
  ///     .get_client()?
  ///     .from_url("https://tinypng.com/images/panda-happy.png")?
  ///     .convert((
  ///          Some(Type::JPEG),
  ///          None,
  ///          None,
  ///       ),
  ///       Some(Color("#FF5733")),
  ///     )?
  ///     .to_file("./converted.webp");
  ///
  ///   Ok(())
  /// }
  /// ```
  pub fn convert<T>(
    self,
    convert_type: (Option<T>, Option<T>, Option<T>),
    transform: Option<Color>,
  ) -> Result<Self, TinifyError>
  where
    T: Into<String> + Copy,
  {
    let types = &[&convert_type.0, &convert_type.1, &convert_type.2];
    let count: Vec<String> = types
      .iter()
      .filter_map(|&val| val.and_then(|x| Some(x.into())))
      .collect();
    let len = count.len();
    let parse_type = match len {
      _ if len >= 2 => serde_json::to_string(&count).unwrap(),
      _ => count.first().unwrap().to_string(),
    };
    let template = if let Some(color) = transform {
      JsonData::new(Convert::new(parse_type), Some(Transform::new(color.0)))
    } else {
      JsonData::new(Convert::new(parse_type), None)
    };

    // Using replace to avoid invalid JSON string.
    let json_string = serde_json::to_string(&template)
      .unwrap()
      .replace("\"convert_type\"", "\"type\"")
      .replace(",\"transform\":null", "")
      .replace("\"[", "[")
      .replace("]\"", "]")
      .replace("\\\"", "\"");
    let response = self
      .request_client
      .post(self.url.as_ref().unwrap())
      .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
      .body(json_string)
      .basic_auth("api", self.key.as_ref())
      .timeout(Duration::from_secs(300))
      .send()?;

    if response.status() == StatusCode::BAD_REQUEST {
      return Err(TinifyError::ClientError);
    }

    self.get_source_from_response(response)
  }

  /// Save the compressed image to a file.
  pub fn to_file<P>(&self, path: P) -> Result<(), TinifyError>
  where
    P: AsRef<Path>,
  {
    let file = File::create(path)
      .map_err(|source| TinifyError::WriteError { source })?;
    let mut reader = BufWriter::new(file);
    reader.write_all(self.buffer.as_ref().unwrap())?;
    reader.flush()?;

    Ok(())
  }

  /// Convert the compressed image to a buffer.
  pub fn to_buffer(&self) -> Vec<u8> {
    self.buffer.as_ref().unwrap().to_vec()
  }

  pub(crate) fn get_source_from_response(
    mut self,
    response: Response,
  ) -> Result<Self, TinifyError> {
    if let Some(location) = response.headers().get("location") {
      let mut url = String::new();

      if !location.is_empty() {
        let slice = str::from_utf8(location.as_bytes()).unwrap();
        url.push_str(slice);
      }

      let get_request = self.request(&url, Method::GET, None);
      let bytes = get_request?.bytes()?.to_vec();
      self.buffer = Some(bytes);
      self.url = Some(url);
    } else {
      let bytes = response.bytes()?.to_vec();
      self.buffer = Some(bytes);
      self.url = None;
    }

    Ok(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use assert_matches::assert_matches;

  #[test]
  fn test_request_error() {
    let source = Source::new(None, None);
    let request = source.request("", Method::GET, None).unwrap_err();

    assert_matches!(request, TinifyError::ReqwestError { .. });
  }
}
