#![feature(async_closure)]
#![feature(ascii_char)]

extern crate clap;

use clap::Parser;
use log::info;
use lumi2::{utils::logging::setup_logging, cli::Args};

pub const VM_VERSION: &str = "2.0.0";

#[tokio::main]
async fn main() {
  if let Err(e) = setup_logging() {
    eprintln!("Error setting logging: {}", e);
    return;
  }

  let args = Args::parse();

  info!("Lumi GPL VM v{}", VM_VERSION);

  match args.command {
    lumi2::cli::Commands::Assemble { input_file } => {
      info!("assembling {} file...", input_file.unwrap_or("".to_string()));
    }
    lumi2::cli::Commands::Run { input_file } => {
      info!("running {} executable...", input_file);
    }
    lumi2::cli::Commands::Console {} => {
      info!("launching REPL console...");
    }
  }
}
