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
use std::fs::File;
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
        Commands::Console { threads } => {
            let num_threads = threads.unwrap_or(num_cpus::get());
            start_repl(num_threads);
        }
        Commands::Run {
            input_file,
            threads,
        } => {
            if let Some(file) = input_file {
                info!("Executing file: {}", file);

                let bytecode = read_binary_file(&file);
                let num_threads = threads.unwrap_or(num_cpus::get());
                start_repl_with_bytecode(bytecode, num_threads);
            } else {
                error!("No input file provided to assemble");
            }
        }
    }
}

fn start_repl(num_threads: usize) {
    let mut repl = repl::REPL::new(num_threads);
    repl.run();
}

fn start_repl_with_bytecode(bytecode: Vec<u8>, num_threads: usize) {
    let mut repl = repl::REPL::new(num_threads);
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
