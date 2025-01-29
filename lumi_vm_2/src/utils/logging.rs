use std::{thread, fs::OpenOptions, fs};
use std::path::PathBuf;
use chrono::format;
use clap::error::Result;
use colored::{ColoredString, Colorize};
use fern::Dispatch;
use log::{Level, LevelFilter};

pub const LUMI_HOME: &str = ".lumi";
pub const LUMI_LOG_FILE: &str = "lumi-log.log";

pub fn get_lumi_home_path() -> PathBuf {
  let home_dir = dirs::home_dir().expect("Could not find home directory");
  home_dir.join(LUMI_HOME)
}

pub fn get_lumi_log_path() -> PathBuf {
  get_lumi_home_path().join(LUMI_LOG_FILE)
}

pub fn init_lumi_home() {
  let lumi_home_path = get_lumi_home_path();
  if !lumi_home_path.exists() {
    fs::create_dir_all(&lumi_home_path).expect("Failed to create lumi home directory");
  }
}

fn colorize_by_level(level: Level) -> ColoredString {
  let level_str = level.to_string();
  let padded_level = format!("{: <5}", level_str);

  match level {
    Level::Error => padded_level.to_string().red(),
    Level::Warn => padded_level.to_string().yellow(),
    Level::Info => padded_level.to_string().green(),
    Level::Debug => padded_level.to_string().blue(),
    Level::Trace => padded_level.to_string().magenta(),
  }
}

pub fn setup_logging() -> Result<(), fern::InitError> {
  let file_config = Dispatch::new()
    .format(|out, message, record| {
      let current_thread = thread::current();
      let thread_name = current_thread.name().unwrap_or("unknown");
      let level = record.level();
      let timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();
      let separator_1 = "---";
      let separator_2 = ":";

      out.finish(format_args!(
        "{} {} {} {} {} {} {}",
        timestamp,
        level,
        separator_1,
        thread_name,
        record.target(),
        separator_2,
        message
      ))
    })
    .level(LevelFilter::Trace)
    .chain(OpenOptions::new()
      .write(true)
      .create(true)
      .append(true)
      .open(get_lumi_log_path())
      .expect("failed to set up a logging chain")
    );

  let stdout_config = Dispatch::new()
    .format(|out, message, record| {
      let current_thread = thread::current();
      let thread_name = current_thread.name().unwrap_or("unknown");
      let level = record.level();
      let colorized_timestamp = chrono::Local::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string()
        .white();
      let colorized_level = colorize_by_level(level);
      let colorized_target = record.target().to_string().blue();
      let colorized_message = message.to_string().bright_white();
      let separator_1 = "---".to_string().white();
      let separator_2 = ":".to_string().white();

      out.finish(format_args!(
        "{} {} {} {} {} {} {}",
        colorized_timestamp,
        colorized_level,
        separator_1,
        format!("[{}]", thread_name).to_string().white(),
        colorized_target,
        separator_2,
        colorized_message
      ))
    })
    .level(LevelFilter::Trace)
    .chain(std::io::stdout());

  Dispatch::new()
    .chain(file_config)
    .chain(stdout_config)
    .apply()
    .expect("failed to dispatch logging chain");

  Ok(())
}
