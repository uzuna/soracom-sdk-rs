use std::error::Error as StdError;
use std::fmt;

//
// refer: https://github.com/BurntSushi/rust-csv/blob/26c737cc85dc2e5fe1df9e7000ab0e3de3203e6b/src/error.rs
//

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
  /// A crate private constructor for `Error`.
  pub(crate) fn new(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
  }

  /// Return the specific type of this error.
  pub fn kind(&self) -> &ErrorKind {
    &self.0
  }
}

#[derive(Debug)]
pub enum ErrorKind {
  JSON(serde_json::error::Error),
  Request(reqwest::Error),
  URL(url::ParseError),
  Env(std::env::VarError),
  HTTP(String),
}

impl From<serde_json::error::Error> for Error {
  fn from(err: serde_json::error::Error) -> Error {
    Error::new(ErrorKind::JSON(err))
  }
}

impl From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Error {
    Error::new(ErrorKind::Request(err))
  }
}

impl From<url::ParseError> for Error {
  fn from(err: url::ParseError) -> Error {
    Error::new(ErrorKind::URL(err))
  }
}
impl From<std::env::VarError> for Error {
  fn from(err: std::env::VarError) -> Error {
    Error::new(ErrorKind::Env(err))
  }
}

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match *self.0 {
      ErrorKind::JSON(ref err) => Some(err),
      ErrorKind::Request(ref err) => Some(err),
      ErrorKind::URL(ref err) => Some(err),
      ErrorKind::Env(ref err) => Some(err),
      ErrorKind::HTTP(_) => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self.0 {
      ErrorKind::JSON(ref err) => err.fmt(f),
      ErrorKind::Request(ref err) => err.fmt(f),
      ErrorKind::URL(ref err) => err.fmt(f),
      ErrorKind::Env(ref err) => err.fmt(f),
      ErrorKind::HTTP(ref err) => write!(f, "HTTP error: {}", err),
    }
  }
}

pub fn new_http_error<T: fmt::Display, B: fmt::Display>(status: T, body: B) -> Error {
  Error::new(ErrorKind::HTTP(format!("{} {}", status, body)))
}
