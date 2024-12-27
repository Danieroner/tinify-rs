use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fmt;
use std::io;
#[cfg(feature = "async")]
use tokio::task;

/// Tinify remote error message received.
#[derive(Serialize, Deserialize, Debug)]
pub struct Upstream {
  pub error: String,
  pub message: String,
}

/// The `TinifyError` enum indicates whether a client or server error occurs.
#[derive(Debug)]
pub enum TinifyError {
  ClientError {
    upstream: Upstream,
  },
  ServerError {
    upstream: Upstream,
  },
  ReqwestError(reqwest::Error),
  ReqwestConvertError(reqwest::header::ToStrError),
  UrlParseError(url::ParseError),
  JsonParseError(serde_json::Error),
  IOError(io::Error),
  #[cfg(feature = "async")]
  TokioError(task::JoinError),
}

impl error::Error for TinifyError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match *self {
      TinifyError::ClientError { .. } => None,
      TinifyError::ServerError { .. } => None,
      TinifyError::ReqwestError(ref source) => Some(source),
      TinifyError::ReqwestConvertError(ref source) => Some(source),
      TinifyError::UrlParseError(ref source) => Some(source),
      TinifyError::JsonParseError(ref source) => Some(source),
      TinifyError::IOError(ref source) => Some(source),
      #[cfg(feature = "async")]
      TinifyError::TokioError(ref source) => Some(source),
    }
  }
}

impl fmt::Display for TinifyError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      TinifyError::ClientError { ref upstream } => {
        write!(f, "Tinify Client Error: {}", upstream.message)
      }
      TinifyError::ServerError { ref upstream } => {
        write!(f, "Tinify Server Error: {}", upstream.message)
      }
      TinifyError::ReqwestError(ref err) => err.fmt(f),
      TinifyError::ReqwestConvertError(ref err) => err.fmt(f),
      TinifyError::UrlParseError(ref err) => err.fmt(f),
      TinifyError::JsonParseError(ref err) => err.fmt(f),
      TinifyError::IOError(ref err) => err.fmt(f),
      #[cfg(feature = "async")]
      TinifyError::TokioError(ref err) => err.fmt(f),
    }
  }
}

impl From<io::Error> for TinifyError {
  fn from(err: io::Error) -> Self {
    TinifyError::IOError(err)
  }
}

impl From<reqwest::Error> for TinifyError {
  fn from(err: reqwest::Error) -> Self {
    TinifyError::ReqwestError(err)
  }
}

impl From<reqwest::header::ToStrError> for TinifyError {
  fn from(err: reqwest::header::ToStrError) -> Self {
    TinifyError::ReqwestConvertError(err)
  }
}

impl From<url::ParseError> for TinifyError {
  fn from(err: url::ParseError) -> Self {
    TinifyError::UrlParseError(err)
  }
}

impl From<serde_json::Error> for TinifyError {
  fn from(err: serde_json::Error) -> Self {
    TinifyError::JsonParseError(err)
  }
}

#[cfg(feature = "async")]
impl From<tokio::task::JoinError> for TinifyError {
  fn from(err: tokio::task::JoinError) -> Self {
    TinifyError::TokioError(err)
  }
}
