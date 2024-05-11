mod repl_commands;

use crate::assembler::program_parsers::parse_program;
use crate::assembler::Assembler;
use crate::scheduler::Scheduler;
use crate::vm::{VMEvent, VM};
use log::{debug, error, info};
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
    history: Vec<String>,
}

impl REPL {
    pub fn new(num_threads: usize, enable_ssh: bool, use_ssh_port: u16) -> REPL {
        let mut vm = VM::new();
        vm.logical_cores = num_threads;

        if enable_ssh {
            info!("Enabling SSH with port: {}", use_ssh_port);
        }

        REPL {
            vm,
            command_buffer: vec![],
            asm: Assembler::new(),
            scheduler: Scheduler::new(),
            history: vec![],
        }
    }

    pub fn load(&mut self, bytecode: Vec<u8>) {
        self.vm.add_bytes(bytecode);
    }

    pub fn execute(&mut self) -> Vec<VMEvent> {
        self.vm.run()
    }

    pub fn execute_command(&mut self, buffer: &str) {
        let string_buffer = buffer.to_string();
        let split = string_buffer.split_whitespace();
        let tokens: Vec<String> = split.map(|s| s.to_string()).collect();

        if let Some(command) = tokens.get(0) {
            let args = tokens[1..].to_vec();
            match command.as_str() {
                "!quit" => {
                    self.command_quit(args);
                }
                "!history" => {
                    self.command_history(args);
                }
                "!clear_program" => {
                    self.command_clear_program(args);
                }
                "!clear_registers" => {
                    self.command_clear_registers(args);
                }
                "!registers" => {
                    self.command_registers(args);
                }
                "!assemble" => {
                    self.command_assemble(args);
                }
                "!symbols" => {
                    self.command_symbols(args);
                }
                "!load" => {
                    self.command_load_program(args);
                }
                "!hex_dump" => {
                    self.command_hex_dump(args);
                }
                "!spawn" => {
                    self.command_spawn(args);
                }
                "!run" => {
                    self.command_run(args);
                }
                "!disassemble" => {
                    self.command_disassemble(args);
                }
                _ => {
                    error!("Unrecognized command");
                }
            }
        }
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
            self.history.push(buffer.to_string());

            if buffer.starts_with("!") {
                self.execute_command(&buffer);
            } else {
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

    // fn get_user_input(&self, prompt: Option<String>) -> Option<String> {
    //     if prompt.is_some() {
    //         print!("{}", prompt.unwrap());
    //     }
    //
    //     let stdin = io::stdin();
    //     let flush_result = io::stdout().flush();
    //     if flush_result.is_err() {
    //         error!("Failed to flush stdout: {:?}", flush_result.err().unwrap());
    //         return None;
    //     }
    //
    //     let mut tmp = String::new();
    //     match stdin.read_line(&mut tmp) {
    //         Ok(user_input) => {
    //             debug!("Read in {} bytes from user input", user_input);
    //             Some(tmp.trim().to_string())
    //         }
    //         Err(err) => {
    //             error!("Failed to read line from user input: {:?}", err);
    //             None
    //         }
    //     }
    // }

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
                info!("Read in total of {} bytes", length);
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

    // /// Reads in a file in string format until EOF.
    // fn get_data_from_file(&mut self, file_path: String) -> Option<String> {
    //     let filename = Path::new(&file_path);
    //     let mut file = match File::open(filename) {
    //         Ok(file) => file,
    //         Err(err) => {
    //             error!(
    //                 "An error occurred when opening file '{}': {:?}",
    //                 filename.to_str().unwrap(),
    //                 err
    //             );
    //             return None;
    //         }
    //     };
    //
    //     let mut contents = String::new();
    //     match file.read_to_string(&mut contents) {
    //         Ok(length) => {
    //             info!("Read in total of {} bytes", length);
    //             Some(contents)
    //         }
    //         Err(err) => {
    //             error!("An error occurred whilst reading file: {:?}", err);
    //             None
    //         }
    //     }
    // }
}
