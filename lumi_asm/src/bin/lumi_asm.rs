use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::sync::mpsc::sync_channel;
use clap::{Arg, Command};
use env_logger::Env;
use log::{error, info};
use pest::Parser;
use lumi_asm::Assembler;

pub fn main() -> Result<(), ()> {
    let matches = Command::new("Lumi Assembler")
      .version("2.0.0")
      .about("Assembles Lumi ASM files into bytecode")
      .author("Lumi")
      .arg(
          Arg::new("input")
            .short('i') // Use a char here instead of &str
            .long("input")
            .value_name("FILE")
            .help("Input Lumi ASM file to assemble")
            .required(true),
      )
      .arg(
          Arg::new("output")
            .short('o') // Use a char here instead of &str
            .long("output")
            .value_name("FILE")
            .help("Output file to write the assembled bytecode to")
            .required(true),
      )
      .arg(
          Arg::new("verbose")
            .short('v') // Use a char here instead of &str
            .long("verbose")
            .help("Enable verbose output"),
      )
      .arg(
        Arg::new("debug")
          .short('d') // Use a char here instead of &str
          .long("debug")
          .help("Enable debug output"),
      )
      .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap(); // Always present because it's required
    let output_path = matches.get_one::<String>("output").unwrap(); // Always present because it's required
    let verbose = matches.contains_id("verbose");
    let debug = matches.contains_id("debug");

    if verbose {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    }

    // Read the input file
    let input_code = match read_file(&input_path) {
        Ok(code) => code,
        Err(err) => {
            error!("Could not read input file {}: {}", input_path, err);
            return Err(());
        }
    };
    
    // let mut context = DebuggerContext::default();
    //     context.load_grammar()

    // Assemble the input code
    let mut assembler = Assembler::new();
    let bytecode = match assembler.assemble(&input_code) {
        Ok(bytecode) => bytecode,
        Err(errors) => {
            for err in errors {
                error!("Assembly error: {:?}", err);
            }
            return Err(());
        }
    };
    
    info!("Successfully assembled {} bytes", bytecode.len());
    
    // Write the bytecode to the output file
    if let Err(err) = write_file(output_path, &bytecode) {
        error!("Could not write output file {}: {}", output_path, err);
        return Err(());
    }
    
    info!("Wrote assembled bytecode to {}", output_path);

    Ok(())
}

// Function to read a file into a string
fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Function to write a byte slice to a file
fn write_file(path: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

// Function to generate a hex representation of bytecode
fn hex_representation(bytecode: &[u8]) -> String {
    bytecode
      .iter()
      .map(|byte| format!("{:02X}", byte))
      .collect::<Vec<String>>()
      .join(" ")
}
