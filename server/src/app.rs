use clap::{App, Arg, SubCommand};

pub fn app() -> App<'static, 'static> {
  App::new("Heimdallr")
    .about(crate_description!())
    .version(crate_version!())
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .value_name("FILE")
        .default_value("./default-config.yaml")
        .help("Sets a custom config file")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("verbose")
        .long("verbose")
        .short("v")
        .multiple(true)
        .help("Increases logging verbosity each use up to 3 times"),
    )
    .arg(
      Arg::with_name("log-dir")
        .long("log-dir")
        .short("l")
        .value_name("DIRECTORY")
        .help("If non-empty, write log files in this directory")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("skip-migrations")
        .long("skip-migrations")
        .help("Skip checking & running database migrations")
    )
    .subcommand(
      SubCommand::with_name("database")
        .about("database subcommands")
        .version(crate_version!())
        .subcommand(
          SubCommand::with_name("setup")
            .arg(Arg::with_name("seed").long("seed").help("Seeds the database with test data"))
        )
    )
}

/// Different types of envor
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Env {
  Development,
  Production,
  Test,
}

/// Gets the current app environment from the env variable `HEIMDALLR_ENV`
pub fn get_environment() -> Env {
  let env_value = std::env::var("HEIMDALLR_ENV")
    .unwrap_or("DEVELOPMENT".to_owned())
    .to_uppercase();

  match &env_value as &str {
    "PRODUCTION" => Env::Production,
    "TEST"       => Env::Test,
    _            => Env::Development
  }
}

/// Checks whether or not the application is running in production.
pub fn is_production() -> bool {
  match get_environment() {
    Env::Production => true,
    _               => false
  }
}

/// Checks whether or not the application is running in development.
pub fn is_development() -> bool {
  match get_environment() {
    Env::Development => true,
    _                => false
  }
}

/// Checks whether or not the application is running in test.
pub fn is_test() -> bool {
  match get_environment() {
    Env::Test => true,
    _         => false
  }
}
