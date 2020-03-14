use crate::error::*;

use log::LevelFilter;

/// Initializes the logging system.
pub fn init<S>(verbosity: u64, log_dir: Option<S>) -> Result<(), HeimdallrError>
  where S: Into<String> + std::fmt::Display {
  let mut base_config = fern::Dispatch::new();

  base_config = match verbosity {
    0 => base_config.level(LevelFilter::Info).level_for("tokio_reactor", LevelFilter::Warn),
    1 => base_config.level(LevelFilter::Debug).level_for("tokio_reactor", LevelFilter::Warn),
    2 => base_config.level(LevelFilter::Debug),
    _ => base_config.level(LevelFilter::Trace),
  };

  if let Some(directory) = log_dir {
    let file_config = fern::Dispatch::new()
      .format(|out, message, record| {
        out.finish(format_args!(
          "{}[{}][{}] {}",
          chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
          record.target(),
          record.level(),
          message
        ))
      })
      .chain(fern::log_file(format!("{}/output.log", directory))?);
    base_config = base_config.chain(file_config);
  }

  let stdout_config = fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(
        format_args!("[{}][{}][{}] {}",
          chrono::Local::now().format("%H:%M"),
          record.target(),
          record.level(),
          message
        ))
    })
    .chain(std::io::stdout());

  base_config.chain(stdout_config).apply()?;
  Ok(())
}
