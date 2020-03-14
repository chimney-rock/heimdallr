extern crate diesel_migrations;

use self::diesel_migrations::run_pending_migrations;
use diesel::prelude::*;
use dotenv;

/// Establishes a connection to the database & starts a transaction.
#[cfg(test)]
pub fn connection() -> PgConnection {
  let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let conn = PgConnection::establish(&url).unwrap();
  run_pending_migrations(&conn).unwrap();
  conn.begin_test_transaction().unwrap();
  conn
}
