#![feature(async_closure)]
#![feature(ascii_char)]

extern crate clap;
extern crate nom;
extern crate nom_supreme;
extern crate num_cpus;

use clap::Parser;
use log::{error, info};
use lumi::cli::{Args, Commands};
use lumi::util::init_lumi_home;
use lumi::util::logging::setup_logging;
use lumi::{assembler, repl};
use russh::server::Server;
use russh::{server, Preferred, SshId};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use base64::Engine;
use russh_keys::decode_secret_key;
use russh_keys::key::KeyPair;
use base64::engine::general_purpose;
use ed25519_dalek::ed25519::signature::rand_core::OsRng;
use ed25519_dalek::SigningKey;
use nom::AsBytes;

pub const VM_VERSION: &str = "1.0.0";

#[tokio::main]
async fn main() {
  init_lumi_home();
  if let Err(e) = setup_logging() {
    eprintln!("Error setting up logging: {}", e);
    return;
  }

  let args = Args::parse();

  info!("Lumi General Programming Language VM v{}", VM_VERSION);
  match args.command {
    Commands::Assemble { input_file } => {
      if let Some(file) = input_file {
        info!("Assembling file: {}", file);
        let mut asm = assembler::Assembler::new();
        asm.assemble_file(&file);
      } else {
        error!("No input file provided to assemble");
      }
    }
    Commands::Console {
      threads,
      enable_ssh,
      ssh_port,
    } => {
      let num_threads = threads.unwrap_or(num_cpus::get());
      let use_ssh_port = ssh_port.unwrap_or(2222);
      info!("Starting console: {}", enable_ssh);
      if enable_ssh {
        start_ssh_server().await;
      }
      start_repl(num_threads, enable_ssh, use_ssh_port);
    }
    Commands::Run {
      input_file,
      threads,
      enable_ssh,
      ssh_port,
    } => {
      if let Some(file) = input_file {
        info!("Executing file: {}", file);

        let bytecode = read_binary_file(&file);
        let num_threads = threads.unwrap_or(num_cpus::get());
        let use_ssh_port = ssh_port.unwrap_or(2222);
        start_repl_with_bytecode(bytecode, num_threads, enable_ssh, use_ssh_port);
      } else {
        error!("No input file provided to assemble");
      }
    }
    Commands::AddSshKey { pub_key_file } => {
      info!("Adding user-defined ssh key...");
      std::process::exit(0);
    }
  }
}

async fn start_ssh_server() {
  info!("Starting ssh server...");
  let t = tokio::spawn(async move {
    let mut config = russh::server::Config {
      server_id: SshId::Standard("SSH-2.0-LumiVM".to_string()),
      inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
      auth_rejection_time: std::time::Duration::from_secs(3),
      auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
      ..Default::default()
    };

    // check if `.lumi` directory exists
    let lumi_dir = Path::new(".lumi");
    if !lumi_dir.exists() {
      std::fs::create_dir(lumi_dir).unwrap();
    }

    // TODO: accept key path from args, resort to known path if not provided
    let keypair_path = Path::new(".lumi/id_ed25519_test_key");
    let key = if keypair_path.exists() {
      let key_content = std::fs::read(&keypair_path).unwrap();
      let key_content_str = std::str::from_utf8(&key_content).unwrap();
      decode_secret_key(&key_content_str, None).expect(
        "Failed to decode secret key. Make sure the key is in the correct format and is valid."
      )
    } else {
      panic!("No key found at path: {:?}", keypair_path);
    };

    config.keys.push(key);
    let config = Arc::new(config);
    let mut sh = lumi::ssh::Server::new();
    // let mut sh = lumi::ssh::Server {
    //   clients: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
    //   id: 0,
    // };
    info!("Starting SSH server...");
    sh.run_on_address(config, ("0.0.0.0", 2222)).await.expect("failed to start ssh server");
  });

  tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
}

fn start_repl(num_threads: usize, enable_ssh: bool, use_ssh_port: u16) {
  let mut repl = repl::REPL::new(num_threads, enable_ssh, use_ssh_port);
  repl.run();
}

fn start_repl_with_bytecode(
  bytecode: Vec<u8>,
  num_threads: usize,
  enable_ssh: bool,
  use_ssh_port: u16,
) {
  let mut repl = repl::REPL::new(num_threads, enable_ssh, use_ssh_port);
  repl.load(bytecode);
  let events = repl.execute();
  for event in &events {
    info!("{:#?}", event);
  }
  repl.run();
}

fn read_binary_file(tmp: &str) -> Vec<u8> {
  let filename = Path::new(tmp);
  match File::open(Path::new(&filename)) {
    Ok(mut file_handle) => {
      let mut contents = vec![];
      match file_handle.read_to_end(&mut contents) {
        Ok(_) => {
          return contents;
        }
        Err(err) => {
          error!("There was an error reading file: {:?}", err);
          std::process::exit(1);
        }
      }
    }
    Err(err) => {
      error!("File not found: {:?}", err);
      std::process::exit(1);
    }
  }
}
