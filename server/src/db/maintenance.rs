use crate::settings::Database as DBSettings;
use super::query_helper::*;
use crate::error::*;
use diesel::*;

// Embed all migrations into the executable.
embed_migrations!();

table! {
  pg_database (datname) {
    datname -> Text,
    datistemplate -> Bool,
  }
}

/// Checks embedded migrations & runs if necessary.
pub fn check_migrations(db_settings: &DBSettings) -> Result<(), HeimdallrError> {
  debug!("Checking database migrations...");
  let connection = super::establish_connection(&db_settings)?;
  embedded_migrations::run_with_output(&connection, &mut std::io::stdout())?;
  Ok(())
}

/// Creates a database if necessary.
pub fn create_database_if_needed(db_settings: &DBSettings) -> Result<(), HeimdallrError> {
  if super::establish_connection(&db_settings).is_err() {
    println!("Database does not exist, attempting to create...");
    let url  = build_default_uri(&db_settings);
    let conn = PgConnection::establish(&url)?;
    create_database(&db_settings.name).execute(&conn)?;
  }

  Ok(())
}

/// Checks whether or not a given database exists.
///
/// # Example
/// ```
/// if database_exists(&connection, "email_hooks").is_err() {
///   println!("Database does not exist!");
/// }
/// ```
fn database_exists(conn: &PgConnection, name: &str) -> QueryResult<bool> {
  use self::pg_database::dsl::*;

  pg_database
    .select(datname)
    .filter(datname.eq(name))
    .filter(datistemplate.eq(false))
    .get_result::<String>(conn)
    .optional()
    .map(|x| x.is_some())
}

/// Checks whether or not a given table exists.
///
/// # Example
/// ```
/// if table_exists(&connection, "heimdallr_dev").is_err() {
///   println!("Uh-Oh Spaghettios!");
/// }
/// ```
fn table_exists(conn: &PgConnection, name: &str) -> QueryResult<bool> {
  use diesel::dsl::sql;

  select(sql::<diesel::sql_types::Bool>(
    &format!("EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = '{}')", name)
  )).get_result(conn)
}

/// Builds a Postgres server uri used to establish a connection when the db does not exist.
fn build_default_uri(db_settings: &DBSettings) -> String {
  format!(
    "postgres://{}:{}@{}:{}/postgres",
    db_settings.username.to_owned(),
    db_settings.password.to_owned(),
    db_settings.host.to_owned(),
    db_settings.port.unwrap_or(5432)
  )
}
