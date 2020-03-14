use heimdallr::prelude::*;
use heimdallr::db::Database;
use heimdallr::services::auth;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // dotenv::dotenv().ok();

  let args = app().get_matches();
  let verbosity = args.occurrences_of("verbose");
  logging::init(verbosity, args.value_of("log-dir"))?;

  // Safe to unwrap without exploding since the arg has a default value
  let settings = Settings::new(args.value_of("config").unwrap())?;

  if let Some(cmd_args) = args.subcommand_matches("database") {
    commands::database::handle(&settings, &args, &cmd_args)?;
  }
  else {
    let database = Database::create_pool(&settings.database)?;
    let handler  = auth::AuthHandler::new(database);

    Server::builder()
      .add_service(handler.service())
      .serve(settings.grpc_listener.address)
      .await?;
  }

  Ok(())
}
