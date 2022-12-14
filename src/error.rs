use reqwest;
use std::io;
use std::fmt;
use std::error;

/// The Tinify API uses HTTP status codes to indicate success or failure.
/// 
/// Status codes in the 4xx range indicate there was a problem with `Client` request.
/// 
/// Status codes in the 5xx indicate a temporary problem with the Tinify API `Server`.
#[derive(Debug)]
pub enum TinifyError {
  ClientError,
  ServerError,
  ReadError { source: io::Error },
  WriteError { source: io::Error },
  IOError(io::Error),
  ReqwestError(reqwest::Error),
}

impl error::Error for TinifyError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match *self {
      TinifyError::ClientError => None,
      TinifyError::ServerError => None,
      TinifyError::ReadError { ref source } => Some(source),
      TinifyError::WriteError { ref source } => Some(source),
      TinifyError::IOError(_) => None,
      TinifyError::ReqwestError(_) => None,
    }
  }
}

impl fmt::Display for TinifyError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      TinifyError::ClientError => {
        write!(f, "There was a problem with the request.")
      },
      TinifyError::ServerError => {
        write!(f, "There is a temporary problem with the Tinify API.")
      },
      TinifyError::ReadError { .. } => {
        write!(f, "Read error")
      },
      TinifyError::WriteError { .. } => {
        write!(f, "Write error")
      },
      TinifyError::IOError(ref err) => {
        err.fmt(f)
      },
      TinifyError::ReqwestError(ref err) => {
        err.fmt(f)
      },
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
