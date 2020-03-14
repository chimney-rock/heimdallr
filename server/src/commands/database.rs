use crate::error::*;

use clap::ArgMatches;

// use crate::db::maintenance::*;
use crate::settings::Settings;

pub fn handle(settings: &Settings, args: &ArgMatches, cmd_args: &ArgMatches) -> Result<(), HeimdallrError> {
  if let Some(matches) = cmd_args.subcommand_matches("setup") {
    setup(&settings, &args, &matches)
  }
  else {
    println!("{}", cmd_args.usage());
    Ok(())
  }
}

/// Initializes the database.
fn setup(_settings: &Settings, _args: &ArgMatches, _cmd_args: &ArgMatches) -> Result<(), HeimdallrError> {
  // create_database_if_needed(&settings.database)?;
  // check_migrations(&settings.database)?;
  Ok(())
}
