use std::{error::Error, fmt};

#[derive(Debug)]
pub enum HeimdallrError {
  ConfigError(config::ConfigError),
  LogError(log::SetLoggerError),
  IOError(std::io::Error),
  DatabaseConnectionError(diesel::ConnectionError),
  R2D2Error(r2d2::Error),
  JwtError(&'static str)
}

impl Error for HeimdallrError {}

impl fmt::Display for HeimdallrError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use HeimdallrError::*;

    match self {
      ConfigError(err)             => write!(f, "Config parse error ({})", err),
      LogError(err)                => write!(f, "Log error ({})", err),
      IOError(err)                 => write!(f, "IO error ({})", err),
      DatabaseConnectionError(err) => write!(f, "Database connection error ({})", err),
      R2D2Error(err)               => write!(f, "Database error ({})", err),
      JwtError(err)                => write!(f, "JWT Error ({})", err)
    }
  }
}

impl From<config::ConfigError> for HeimdallrError {
  fn from(err: config::ConfigError) -> HeimdallrError {
    HeimdallrError::ConfigError(err)
  }
}

impl From<log::SetLoggerError> for HeimdallrError {
  fn from(err: log::SetLoggerError) -> HeimdallrError {
    HeimdallrError::LogError(err)
  }
}

impl From<std::io::Error> for HeimdallrError {
  fn from(err: std::io::Error) -> HeimdallrError {
    HeimdallrError::IOError(err)
  }
}

impl From<diesel::ConnectionError> for HeimdallrError {
  fn from(err: diesel::ConnectionError) -> HeimdallrError {
    HeimdallrError::DatabaseConnectionError(err)
  }
}

impl From<r2d2::Error> for HeimdallrError {
  fn from(err: r2d2::Error) -> HeimdallrError {
    HeimdallrError::R2D2Error(err)
  }
}
