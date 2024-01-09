use crate::util::{to_hex, visualize_program};
use crate::vm::VM;
use log::{debug, error, info};
use std::io;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum WatchType {
    Memory(usize),
    Register(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WatchVariable {
    pub(crate) watch_type: WatchType,
    pub(crate) last_value: i32,
}

impl VM {
    fn add_watch_variable(&mut self, watch_type: WatchType) {
        let initial_value = match watch_type {
            WatchType::Memory(addr) => self.heap[addr] as i32,
            WatchType::Register(index) => self.registers[index],
        };

        self.watch_variables.insert(
            watch_type.clone(),
            WatchVariable {
                watch_type,
                last_value: initial_value,
            },
        );
    }

    /// Opcode: PRTS
    ///
    /// Arguments: 24-bit heap offset
    pub(crate) fn system_execute_print_string(&mut self) {
        let starting_offset = self.next_24_bits() as usize;
        let mut ending_offset = starting_offset;
        let slice = self.ro_data.as_slice();

        debug!("Starting offset: {}", starting_offset);
        debug!("Ending offset: {}", ending_offset);
        debug!("Slice: {:?}", slice);

        while slice[ending_offset] != 0 {
            ending_offset += 1;
        }

        let result = std::str::from_utf8(&slice[starting_offset..ending_offset]);
        match result {
            Ok(s) => {
                info!("{}", s);
            }
            Err(e) => {
                error!("Error decoding string for prts instruction: {:#?}", e);
            }
        }
    }

    /// Calls a subroutine.
    /// Creates a return destination.
    /// Gets the address destination to jump to.
    /// Pushes the return address to stack.
    ///
    /// Opcode: CALL
    ///
    /// Arguments: 16-bit destination (direct PC address)
    pub(crate) fn system_execute_call(&mut self) {
        let return_destination = self.pc + 3;
        let destination = self.next_16_bits();
        self.stack.push(return_destination as i32);
        self.stack.push(self.bp as i32);
        self.bp = self.sp;
        self.pc = destination as usize;
    }

    /// Opcode: RET
    ///
    /// Arguments: none
    pub(crate) fn system_execute_return(&mut self) {
        self.sp = self.bp;
        self.bp = self.stack.pop().unwrap() as usize;
        self.pc = self.stack.pop().unwrap() as usize;
    }

    /// Opcode: BKPT
    ///
    /// Arguments: none
    pub(crate) fn system_execute_breakpoint(&mut self) -> bool {
        loop {
            info!(
                "Breakpoint hit at PC: {}. Enter 'c' to continue, 's' to step, 'w' to watch a variable, 'i' to inspect, or 'q' to quit:",
                self.pc
            );
            visualize_program(&self.program, Some(self.pc));
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            print!("[0x{}] >>> ", to_hex(self.pc));
            io::stdout().flush().expect("Failed to flush stdout");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim() {
                "c" => return false,
                "s" => return true,
                "w" => {
                    print!(">>> watch variable: ");
                    input = "".to_string();
                    io::stdout().flush().expect("Failed to flush stdout");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let split = input.split_whitespace();
                    let tokens: Vec<String> = split.map(|s| s.to_string()).collect();
                    let watch_type;
                    if tokens[0].eq_ignore_ascii_case("memory") {
                        watch_type = WatchType::Memory(
                            usize::from_str(&tokens[1].trim())
                                .expect("Failed to parse usize from str"),
                        );
                    } else if tokens[0].eq_ignore_ascii_case("register") {
                        watch_type = WatchType::Register(
                            usize::from_str(&tokens[1].trim())
                                .expect("Failed to parse usize from str"),
                        );
                    } else {
                        info!("Unknown watch type provided");
                        continue;
                    }
                    self.add_watch_variable(watch_type);
                    return false;
                }
                "i" => {
                    print!(">>> inspect (r/h/s): ");
                    input = "".to_string();
                    io::stdout().flush().expect("Failed to flush stdout");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let split = input.split_whitespace();
                    let tokens: Vec<String> = split.map(|s| s.to_string()).collect();
                    info!("TOKENS: {:?}", tokens);
                    let value =
                        usize::from_str(&tokens[1].trim()).expect("Failed to parse usize from str");
                    if tokens[0].eq_ignore_ascii_case("r") {
                        info!("Register[0x{}]: {}", to_hex(value), self.registers[value]);
                    } else if tokens[0].eq_ignore_ascii_case("h") {
                        info!("Heap[0x{}]: {}", to_hex(value), self.heap[value]);
                    } else if tokens[0].eq_ignore_ascii_case("s") {
                        info!("Stack[0x{}]: {}", to_hex(value), self.stack[value]);
                    }
                    continue;
                }
                "q" => {
                    std::process::exit(0);
                }
                _ => error!(
                    "Invalid input, please enter 'c' to continue, 's' to step, or 'q' to quit."
                ),
            }
        }
    }
}
