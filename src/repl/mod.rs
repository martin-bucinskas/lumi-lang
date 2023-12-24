use crate::assembler::program_parsers::parse_program;
use crate::assembler::Assembler;
use crate::scheduler::Scheduler;
use crate::util::visualize_program;
use crate::vm::{VMEvent, VM};
use log::{debug, error, info, log};
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

/// Core REPL structure
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
    scheduler: Scheduler,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
            asm: Assembler::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn load(&mut self, bytecode: Vec<u8>) {
        self.vm.add_bytes(bytecode);
    }

    pub fn execute(&mut self) -> Vec<VMEvent> {
        self.vm.run()
    }

    pub fn run(&mut self) {
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".program" => {
                    info!("Currently loaded program:");
                    visualize_program(self.vm.get_program(), None);
                    continue;
                }
                ".clear_program" => {
                    info!("Clearing the program vector...");
                    self.vm.get_program().clear();
                    continue;
                }
                ".registers" => {
                    info!("Listing registers and their contents:");
                    info!("{:#?}", self.vm.get_registers());
                    continue;
                }
                ".history" => {
                    for command in &self.command_buffer {
                        info!("{}", command);
                    }
                    continue;
                }
                ".quit" => {
                    info!("Exiting...");
                    std::process::exit(0);
                }
                ".assemble" => {
                    let file_to_assemble =
                        self.get_user_input(Some("File to assemble: ".to_string()));
                    if file_to_assemble.is_none() {
                        error!("Received no input");
                        continue;
                    }

                    match self.asm.assemble_file(&file_to_assemble.unwrap()) {
                        true => info!("File successfully assembled"),
                        false => error!("Failed to assemble file"),
                    };
                }
                ".run" => {
                    let file_to_run = self.get_user_input(Some("File to run: ".to_string()));
                    if file_to_run.is_none() {
                        error!("Received no input");
                        continue;
                    }

                    let binary = self.get_binary_data_from_file(file_to_run.unwrap());
                    if binary.is_none() {
                        error!("Failed to read file");
                        continue;
                    }

                    self.vm.get_program().append(&mut binary.unwrap());
                    let events = self.vm.run();
                    for event in &events {
                        info!("{:#?}", event);
                        // println!("{:#?}", event);
                    }
                }
                ".spawn" => {
                    let file_to_run = self.get_user_input(Some("File to run: ".to_string()));
                    if file_to_run.is_none() {
                        error!("Received no input");
                        continue;
                    }

                    let binary = self.get_binary_data_from_file(file_to_run.unwrap());
                    if binary.is_none() {
                        error!("Failed to read file");
                        continue;
                    }

                    self.vm.get_program().append(&mut binary.unwrap());
                    let _events = self.scheduler.get_thread(self.vm.clone());
                    continue;
                }
                _ => {
                    let parsed_program = parse_program(buffer);
                    if parsed_program.is_err() {
                        error!("Unable to parse input: {:?}", parsed_program.unwrap_err());
                        continue;
                    }

                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes(&self.asm.symbols);

                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }

    fn get_user_input(&self, prompt: Option<String>) -> Option<String> {
        if prompt.is_some() {
            print!("{}", prompt.unwrap());
        }

        let stdin = io::stdin();
        let flush_result = io::stdout().flush();
        if flush_result.is_err() {
            error!("Failed to flush stdout: {:?}", flush_result.err().unwrap());
            return None;
        }

        let mut tmp = String::new();
        match stdin.read_line(&mut tmp) {
            Ok(user_input) => {
                debug!("Read in {} bytes from user input", user_input);
                Some(tmp.trim().to_string())
            }
            Err(err) => {
                error!("Failed to read line from user input: {:?}", err);
                None
            }
        }
    }

    /// Reads in a file in binary format until EOF.
    fn get_binary_data_from_file(&mut self, file_path: String) -> Option<Vec<u8>> {
        let filename = Path::new(&file_path);
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                error!("An error occurred when opening file: {:?}", err);
                return None;
            }
        };

        let mut contents: Vec<u8> = vec![];
        match file.read_to_end(&mut contents) {
            Ok(length) => {
                debug!("Read in total of {} bytes", length);
                Some(contents)
            }
            Err(err) => {
                error!(
                    "An error occurred whilst reading binary data from file: {:?}",
                    err
                );
                None
            }
        }
    }

    /// Reads in a file in string format until EOF.
    fn get_data_from_file(&mut self, file_path: String) -> Option<String> {
        let filename = Path::new(&file_path);
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                error!(
                    "An error occurred when opening file '{}': {:?}",
                    filename.to_str().unwrap(),
                    err
                );
                return None;
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(length) => {
                debug!("Read in total of {} bytes", length);
                Some(contents)
            }
            Err(err) => {
                error!("An error occurred whilst reading file: {:?}", err);
                None
            }
        }
    }
}
