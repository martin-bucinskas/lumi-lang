pub mod assembler;
mod cli;
pub mod instruction;
pub mod repl;
mod scheduler;
mod util;
pub mod vm;

extern crate clap;
extern crate nom;
extern crate nom_supreme;

use crate::cli::{Args, Commands};
use crate::util::init_lumi_home;
use crate::util::logging::setup_logging;
use clap::Parser;
use colored::{ColoredString, Colorize};
use fern::Dispatch;
use log::{error, info, Level, LevelFilter};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;

pub const VM_VERSION: &str = "1.0.0";

fn main() {
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
        Commands::Console {} => {
            start_repl();
        }
        Commands::Run { input_file } => {
            if let Some(file) = input_file {
                info!("Executing file: {}", file);

                let bytecode = read_binary_file(&file);
                start_repl_with_bytecode(bytecode);
            } else {
                error!("No input file provided to assemble");
            }
        }
    }
}

fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

fn start_repl_with_bytecode(bytecode: Vec<u8>) {
    let mut repl = repl::REPL::new();
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
