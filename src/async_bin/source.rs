use crate::convert::Convert;
use crate::error::TinifyError;
use crate::error::Upstream;
use crate::resize::Resize;
use crate::transform::Transform;
use crate::Operations;
use crate::SourceUrl;
use crate::API_ENDPOINT;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client as ReqwestClient;
use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str;
use std::time::Duration;
use tokio::task;
use url::Url;

#[derive(Debug)]
pub struct Source {
  key: Option<String>,
  buffer: Option<Vec<u8>>,
  output: Option<String>,
  reqwest_client: ReqwestClient,
  operations: Operations,
}

impl Source {
  pub(crate) fn new(key: Option<&str>) -> Self {
    let key = key.map(|val| val.into());
    let reqwest_client = ReqwestClient::new();
    let operations = Operations {
      convert: None,
      resize: None,
      transform: None,
    };

    Self {
      key,
      buffer: None,
      output: None,
      reqwest_client,
      operations,
    }
  }

  async fn get_source_from_response(
    mut self,
    buffer: Option<&[u8]>,
    json: Option<Value>,
  ) -> Result<Self, TinifyError> {
    let parse = Url::parse(API_ENDPOINT)?;
    let url = parse.join("/shrink")?;
    let compressed_image = if let Some(json) = json {
      self
        .reqwest_client
        .post(url)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(json.to_string())
        .basic_auth("api", self.key.as_ref())
        .timeout(Duration::from_secs(300))
        .send()
        .await?
    } else {
      self
        .reqwest_client
        .post(url)
        .body(buffer.unwrap().to_vec())
        .basic_auth("api", self.key.as_ref())
        .timeout(Duration::from_secs(300))
        .send()
        .await?
    };

    match compressed_image.status() {
      StatusCode::CREATED => {
        if let Some(location) = compressed_image.headers().get("location") {
          let location = location.to_str()?.to_string();
          let bytes = self
            .reqwest_client
            .get(&location)
            .timeout(Duration::from_secs(300))
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();

          self.buffer = Some(bytes);
          self.output = Some(location);

          Ok(self)
        } else {
          let upstream = Upstream {
            error: "Empty".to_string(),
            message: "The location of the compressed image is empty."
              .to_string(),
          };
          Err(TinifyError::ServerError { upstream })
        }
      }
      StatusCode::UNAUTHORIZED | StatusCode::UNSUPPORTED_MEDIA_TYPE => {
        let upstream: Upstream =
          serde_json::from_str(&compressed_image.text().await?)?;
        Err(TinifyError::ClientError { upstream })
      }
      _ => {
        let upstream: Upstream =
          serde_json::from_str(&compressed_image.text().await?)?;
        Err(TinifyError::ServerError { upstream })
      }
    }
  }

  #[allow(clippy::wrong_self_convention)]
  pub(crate) async fn from_buffer(
    self,
    buffer: &[u8],
  ) -> Result<Self, TinifyError> {
    self.get_source_from_response(Some(buffer), None).await
  }

  #[allow(clippy::wrong_self_convention)]
  pub(crate) async fn from_file<P>(self, path: P) -> Result<Self, TinifyError>
  where
    P: AsRef<Path>,
  {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(reader.capacity());
    reader.read_to_end(&mut buffer)?;

    self.get_source_from_response(Some(&buffer), None).await
  }

  #[allow(clippy::wrong_self_convention)]
  pub(crate) async fn from_url<P>(self, path: P) -> Result<Self, TinifyError>
  where
    P: AsRef<str> + Into<String>,
  {
    let json = json!({
      "source": SourceUrl { url: path.into() },
    });

    self.get_source_from_response(None, Some(json)).await
  }

  /// Resize the current compressed image.
  pub fn resize(mut self, resize: Resize) -> Result<Self, TinifyError> {
    self.operations.resize = Some(resize);
    Ok(self)
  }

  /// Convert the current compressed image.
  pub fn convert(mut self, convert: Convert) -> Result<Self, TinifyError> {
    self.operations.convert = Some(convert);
    Ok(self)
  }

  /// Transform the current compressed image.
  pub fn transform(
    mut self,
    transform: Transform,
  ) -> Result<Self, TinifyError> {
    self.operations.transform = Some(transform);
    Ok(self)
  }

  async fn run_operations(&mut self) -> Result<(), TinifyError> {
    let operations = serde_json::to_string(&self.operations)?;

    if let Some(output) = self.output.take() {
      let response = self
        .reqwest_client
        .post(output)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(operations)
        .basic_auth("api", self.key.as_ref())
        .timeout(Duration::from_secs(300))
        .send()
        .await?;

      match response.status() {
        StatusCode::OK => {
          let bytes = response.bytes().await?.to_vec();

          self.buffer = Some(bytes);

          Ok(())
        }
        StatusCode::BAD_REQUEST
        | StatusCode::UNAUTHORIZED
        | StatusCode::UNSUPPORTED_MEDIA_TYPE => {
          let upstream: Upstream =
            serde_json::from_str(&response.text().await?)?;
          Err(TinifyError::ClientError { upstream })
        }
        StatusCode::SERVICE_UNAVAILABLE => {
          let upstream: Upstream =
            serde_json::from_str(&response.text().await?)?;
          Err(TinifyError::ServerError { upstream })
        }
        _ => unreachable!(),
      }
    } else {
      let upstream = Upstream {
        error: "Empty".to_string(),
        message: "Output of the compressed image is empty.".to_string(),
      };
      Err(TinifyError::ClientError { upstream })
    }
  }

  /// Save the current compressed image to a file.
  pub async fn to_file<P>(&mut self, path: P) -> Result<(), TinifyError>
  where
    P: AsRef<Path> + Send + 'static,
  {
    if self.operations.convert.is_some()
      || self.operations.resize.is_some()
      || self.operations.transform.is_some()
    {
      self.run_operations().await?;
    }

    if let Some(ref buffer) = self.buffer {
      let file = task::spawn_blocking(move || File::create(path)).await??;
      let mut reader = BufWriter::new(file);
      reader.write_all(buffer)?;
      reader.flush()?;
    }

    Ok(())
  }

  /// Save the current compressed image to a buffer.
  pub async fn to_buffer(&mut self) -> Result<Vec<u8>, TinifyError> {
    if self.operations.convert.is_some()
      || self.operations.resize.is_some()
      || self.operations.transform.is_some()
    {
      self.run_operations().await?;
    }

    if let Some(buffer) = self.buffer.as_ref() {
      Ok(buffer.to_vec())
    } else {
      let upstream = Upstream {
        error: "Empty".to_string(),
        message: "Buffer of the compressed image is empty.".to_string(),
      };
      Err(TinifyError::ClientError { upstream })
    }
  }
}
