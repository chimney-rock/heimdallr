mod schema;
pub use schema::*;

#[cfg(test)]
pub(crate) mod test_helpers;

use crate::settings::Database as DBSettings;
use crate::error::*;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// Establishes a single-use connection to the database
pub fn establish_connection(db_settings: &DBSettings) -> Result<PgConnection, HeimdallrError> {
  use diesel::prelude::*;
  Ok(PgConnection::establish(&build_uri(&db_settings))?)
}

pub(crate) fn build_uri(db_settings: &DBSettings) -> String {
  format!(
    "postgres://{}:{}@{}:{}/{}",
    db_settings.username.to_owned(),
    db_settings.password.to_owned(),
    db_settings.host.to_owned(),
    db_settings.port.unwrap_or(5432),
    db_settings.name.to_owned()
  )
}

#[derive(Clone)]
pub struct Database {
  pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
  pub fn create_pool(db_settings: &DBSettings) -> Result<Self, HeimdallrError> {
    let manager = ConnectionManager::<PgConnection>::new(build_uri(&db_settings));
    let pool = Pool::builder().build(manager)?;
    Ok(Database { pool })
  }
}
