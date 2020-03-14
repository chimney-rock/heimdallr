#![recursion_limit = "256"]
#![allow(dead_code)]

#[macro_use]
extern crate clap;

#[macro_use]
extern crate diesel;

pub mod app;
pub mod commands;
pub mod db;
pub mod error;
pub mod logging;
pub mod jwt;
pub mod services;
pub mod settings;

pub mod prelude {
  pub use crate::app::*;
  pub use crate::commands;
  pub use crate::error::*;
  pub use crate::logging;
  pub use crate::settings::*;
  pub use crate::services;
}
