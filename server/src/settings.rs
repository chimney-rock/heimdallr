use config::{Config, Environment, File};
use serde::Deserialize;
use std::net::SocketAddr;
use crate::error::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  pub grpc_listener: Listener,
  pub database: Database
}

#[derive(Debug, Deserialize, Clone)]
pub struct Listener {
  pub address: SocketAddr,
  pub backlog: Option<i16>,
  pub workers: Option<i16>,

  pub private_key: Option<String>,
  pub cert: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
  pub name: String,
  pub host: String,
  pub port: Option<u16>,
  pub username: String,
  pub password: String,
  pub pool: Option<usize>
}

impl Settings {
  pub fn new<S>(config_file: S) -> Result<Self, HeimdallrError>
    where S: Into<String> {
    let mut cfg = Config::new();

    cfg.merge(File::with_name(&config_file.into()).required(false))?;
    cfg.merge(Environment::with_prefix("heimdallr").separator("_"))?;

    // Deserialize and freeze the entire configuration
    cfg.try_into().map_err(|err| { err.into() })
  }
}
