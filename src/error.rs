use reqwest::StatusCode;
use std::process;
use std::fmt;

pub enum TinifyException {
  AccountException,
  ClientException,
  ServerException,
}

impl fmt::Display for TinifyException {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TinifyException::AccountException => {
        write!(
          fmt,
          "There was a problem with your API key or with your API account.",
        )
      }
      TinifyException::ClientException => {
        write!(
          fmt,
          "The request could not be completed because of a problem with the submitted data.",
        )
      }
      TinifyException::ServerException => {
        write!(
          fmt,
          "The request could not be completed because of a temporary problem with the Tinify API.",
        )
      }
    }
  }
}

pub fn exit_error(
  type_exception: TinifyException,
  status_code: &StatusCode,
) {
  eprintln!(
    "{} status: {}",
    type_exception.to_string(),
    &status_code,
  );
  process::exit(1);
}
