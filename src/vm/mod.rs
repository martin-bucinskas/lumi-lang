mod arithmetic;
mod bitwise;
mod comparison;
mod control;
mod logical;
mod memory;
mod system;

use crate::instruction::Opcode;
use crate::util::header_utils::{LUMI_HEADER_LENGTH, LUMI_HEADER_PREFIX};
use crate::vm::system::{WatchType, WatchVariable};
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, Utc};
use log::{debug, error, info};
use std::collections::HashMap;
use std::io::Cursor;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VMEventType {
    Start,
    GracefulStop { code: u32 },
    Crash { code: u32 },
}

pub enum ExecutionStatus {
    Continue,
    BreakpointHit,
    Done(u32),
}

impl VMEventType {
    pub fn stop_code(&self) -> u32 {
        match &self {
            VMEventType::Start => 0,
            VMEventType::GracefulStop { code } => *code,
            VMEventType::Crash { code } => *code,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMEvent {
    pub event: VMEventType,
    at: DateTime<Utc>,
    application_id: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    /// Array simulating hardware registers
    pub registers: [i32; 32],
    /// Array simulating floating point hardware registers
    pub float_registers: [f64; 32],
    /// Program counter tracking byte execution
    pub(crate) pc: usize,
    /// Bytecode of the program being run
    pub program: Vec<u8>,
    /// The VMs heap memory
    pub heap: Vec<u8>,
    /// The VMs stack memory
    pub stack: Vec<i32>,
    /// Remainder of modulo division operations
    pub(crate) remainder: u32,
    /// Result of last comparison operation
    pub(crate) equal_flag: bool,
    /// List of events for a VM
    pub(crate) events: Vec<VMEvent>,
    /// Contains read-only section data
    pub(crate) ro_data: Vec<u8>,
    /// Loop counter
    pub(crate) loop_counter: usize,
    /// Stack pointer, keeps track of where currently in the stack we are
    pub(crate) sp: usize,
    /// Frame pointer, keeps track of current frame pointer
    pub(crate) bp: usize,
    /// Debugger watched variables
    pub(crate) watch_variables: HashMap<WatchType, WatchVariable>,
    /// Unique randomly generated UUID for identifying this VM
    pub id: Uuid,
    /// Amount of cores detected
    pub logical_cores: usize,
}

impl VM {
    pub fn get_program(&mut self) -> &mut Vec<u8> {
        &mut self.program
    }

    pub fn get_registers(&self) -> &[i32; 32] {
        &self.registers
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn add_bytes(&mut self, mut bytes: Vec<u8>) {
        self.program.append(&mut bytes);
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 32],
            float_registers: [0.0; 32],
            pc: 0,
            program: vec![],
            heap: vec![],
            stack: vec![],
            remainder: 0,
            equal_flag: false,
            events: vec![],
            ro_data: vec![],
            loop_counter: 0,
            sp: 0,
            bp: 0,
            watch_variables: HashMap::new(),
            id: Uuid::new_v4(),
            logical_cores: num_cpus::get(),
        }
    }

    fn read_ro_data(&mut self) {
        // the read_only data offset size (32 bits wide)
        const OFFSET_SIZE: usize = 4;

        let start_index = LUMI_HEADER_LENGTH + OFFSET_SIZE + 1;
        let end_index = start_index + self.get_starting_offset();
        debug!(
            "Read-only data start_index: {}, end_index: {}",
            start_index, end_index
        );
        self.ro_data = self.program[start_index..end_index].to_vec();
    }

    /// Loop as long as instructions can be executed
    pub fn run(&mut self) -> Vec<VMEvent> {
        self.events.push(VMEvent {
            event: VMEventType::Start,
            at: Utc::now(),
            application_id: self.id,
        });

        if !self.verify_header() {
            self.events.push(VMEvent {
                event: VMEventType::Crash { code: 1 },
                at: Utc::now(),
                application_id: self.id,
            });
            error!("Not a LUMI header, skipping execution");
            return self.events.clone();
        }
        // move pc past header
        self.pc = LUMI_HEADER_LENGTH + 1 + 4 + self.get_starting_offset();
        debug!("Code start: {}", self.pc);

        self.read_ro_data();

        let mut is_done = None;
        let mut in_step_mode = false;
        while is_done.is_none() {
            match self.execute_instruction() {
                ExecutionStatus::Continue => {
                    if in_step_mode {
                        in_step_mode = self.system_execute_breakpoint();
                    }
                }
                ExecutionStatus::BreakpointHit => {
                    in_step_mode = self.system_execute_breakpoint();
                }
                ExecutionStatus::Done(code) => {
                    is_done = Some(code);
                }
            }
        }
        self.events.push(VMEvent {
            event: VMEventType::GracefulStop {
                code: is_done.unwrap(),
            },
            at: Utc::now(),
            application_id: self.id,
        });
        self.events.clone()
    }

    /// Executes one instruction to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Executes a single instruction based on PC
    fn execute_instruction(&mut self) -> ExecutionStatus {
        if self.pc >= self.program.len() {
            return ExecutionStatus::Done(1);
        }

        // check for watch variable changes
        for watch_var in self.watch_variables.values_mut() {
            let current_value = match watch_var.watch_type {
                WatchType::Memory(addr) => self.heap[addr] as i32,
                WatchType::Register(index) => self.registers[index],
            };

            if current_value != watch_var.last_value {
                info!("Watch variable changed: {:?}", watch_var);
                watch_var.last_value = current_value;
            }
        }

        // visualize_program(&self.program, Some(self.pc));
        let opcode = self.decode_opcode();
        debug!("PC: {:?}", self.pc);
        debug!("Opcode: {:?}", opcode);
        debug!("Equal flag: {}", self.equal_flag);
        match opcode {
            Opcode::HLT => {
                info!("HLT encountered, stopping execution...");
                return ExecutionStatus::Done(0);
            }
            Opcode::BKPT => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
                return ExecutionStatus::BreakpointHit;
            }
            Opcode::LOAD => {
                self.memory_execute_load();
            }
            Opcode::ADD => {
                self.arithmetic_execute_add();
            }
            Opcode::SUB => {
                self.arithmetic_execute_sub();
            }
            Opcode::MUL => {
                self.arithmetic_execute_mul();
            }
            Opcode::DIV => {
                self.arithmetic_execute_div();
            }
            Opcode::JMP => {
                self.control_execute_jump();
            }
            Opcode::JMPF => {
                self.control_execute_jump_forward();
            }
            Opcode::JMPB => {
                self.control_execute_jump_backward();
            }
            Opcode::EQ => {
                self.comparison_execute_equal();
            }
            Opcode::NEQ => {
                self.comparison_execute_not_equal();
            }
            Opcode::GT => {
                self.comparison_execute_greater_than();
            }
            Opcode::LT => {
                self.comparison_execute_less_than();
            }
            Opcode::GTE => {
                self.comparison_execute_greater_than_or_equal();
            }
            Opcode::LTE => {
                self.comparison_execute_less_than_or_equal();
            }
            Opcode::JMPE => {
                self.control_execute_jump_if_equal();
            }
            Opcode::DJMP => {
                self.control_execute_direct_jump();
            }
            Opcode::DJMPE => {
                self.control_execute_direct_jump_if_equal();
            }
            Opcode::ALOC => {
                self.memory_execute_allocate();
            }
            Opcode::INC => {
                self.arithmetic_execute_increment();
            }
            Opcode::DEC => {
                self.arithmetic_execute_decrement();
            }
            Opcode::PRTS => {
                self.system_execute_print_string();
            }
            Opcode::AND => {
                self.logical_execute_and();
            }
            Opcode::OR => {
                self.logical_execute_or();
            }
            Opcode::XOR => {
                self.logical_execute_xor();
            }
            Opcode::NOT => {
                self.logical_execute_not();
            }
            Opcode::LUI => {
                self.memory_execute_load_upper_immediate();
            }
            Opcode::SHL => {
                self.bitwise_execute_shift_left();
            }
            Opcode::SHR => {
                self.bitwise_execute_shift_right();
            }
            Opcode::LOADM => {
                self.memory_execute_load_memory();
            }
            Opcode::SETM => {
                self.memory_execute_set_memory();
            }
            Opcode::PUSH => {
                self.memory_execute_push_to_stack();
            }
            Opcode::POP => {
                self.memory_execute_pop_from_stack();
            }
            Opcode::LOOP => {
                self.control_execute_loop();
            }
            Opcode::CLOOP => {
                self.control_execute_create_loop();
            }
            Opcode::CALL => {
                self.system_execute_call();
            }
            Opcode::RET => {
                self.system_execute_return();
            }
            Opcode::LOADF64 => {
                self.memory_execute_load_f64();
            }
            Opcode::ADDF64 => {
                self.arithmetic_execute_add_f64();
            }
            Opcode::SUBF64 => {
                self.arithmetic_execute_sub_f64();
            }
            Opcode::MULF64 => {
                self.arithmetic_execute_mul_f64();
            }
            Opcode::DIVF64 => {
                self.arithmetic_execute_div_f64();
            }
            Opcode::EQF64 => {
                self.comparison_execute_equal_f64();
            }
            Opcode::NEQF64 => {
                self.comparison_execute_not_equal_f64();
            }
            Opcode::GTF64 => {
                self.comparison_execute_greater_than_f64();
            }
            Opcode::GTEF64 => {
                self.comparison_execute_greater_than_or_equal_f64();
            }
            Opcode::LTF64 => {
                self.comparison_execute_less_than_f64();
            }
            Opcode::LTEF64 => {
                self.comparison_execute_less_than_or_equal_f64();
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::IGL => {
                error!("Illegal instruction encountered");
                return ExecutionStatus::Done(1);
            }
        };
        ExecutionStatus::Continue
    }

    pub(crate) fn decode_opcode(&mut self) -> Opcode {
        let byte = self.program[self.pc];
        let opcode = Opcode::from(byte);
        self.pc += 1;
        opcode
    }

    pub(crate) fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    /// Gets the next 16 bits from program. Uses little-endian format.
    pub(crate) fn next_16_bits(&mut self) -> u16 {
        let result = (self.program[self.pc] as u16) | ((self.program[self.pc + 1] as u16) << 8);
        self.pc += 2;
        result
    }

    /// Gets the next 24 bits from program. Uses little-endian format.
    pub(crate) fn next_24_bits(&mut self) -> u32 {
        let result = (self.program[self.pc] as u32)
            | ((self.program[self.pc + 1] as u32) << 8)
            | ((self.program[self.pc + 2] as u32) << 16);
        self.pc += 3;
        result
    }

    pub(crate) fn verify_header(&self) -> bool {
        if self.program[0..4] != LUMI_HEADER_PREFIX {
            return false;
        }

        true
    }

    pub(crate) fn get_starting_offset(&self) -> usize {
        let mut rdr =
            Cursor::new(&self.program[LUMI_HEADER_LENGTH + 1..LUMI_HEADER_LENGTH + 1 + 4]);
        rdr.read_u32::<LittleEndian>().unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::header_utils::get_lumi_header;

    fn get_test_vm() -> VM {
        VM::new()
    }

    #[test]
    fn test_create_vm() {
        let test_vm = get_test_vm();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![5, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.pc, 70);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = get_test_vm();
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![200, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.pc, 70);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = get_test_vm();
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![0, 0, 244, 1]); // using little-endian -> [LOAD, register 0, 244 * 2^0, 1 * 2^8]
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm();
        /*
        LOAD $0 #500
        LOAD $1 #25
        ADD $0 $1 $2 -- adds registers $0 and $1 and saves the output to $2
         */
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![0, 0, 244, 1, 0, 1, 25, 0, 1, 0, 1, 2, 5, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
        assert_eq!(test_vm.registers[1], 25);
        assert_eq!(test_vm.registers[2], 525);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = get_test_vm();
        /*
        LOAD $0 #500
        LOAD $1 #25
        SUB $0 $1 $2 -- subtracts registers $0 and $1 and saves the output to $2
         */
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![0, 0, 244, 1, 0, 1, 25, 0, 2, 0, 1, 2, 5, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
        assert_eq!(test_vm.registers[1], 25);
        assert_eq!(test_vm.registers[2], 475);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = get_test_vm();
        /*
        LOAD $0 #2
        LOAD $1 #10
        MUL $0 $1 $2 -- multiplies registers $0 and $1 and saves the output to $2
         */
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![0, 0, 2, 0, 0, 1, 10, 0, 3, 0, 1, 2, 5, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.registers[0], 2);
        assert_eq!(test_vm.registers[1], 10);
        assert_eq!(test_vm.registers[2], 20);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = get_test_vm();
        /*
        LOAD $0 #8
        LOAD $1 #5
        DIV $0 $1 $2 -- divides registers $0 and $1 and saves the output to $2 and remainder in remainder register
         */
        let mut bytecode = vec![];
        bytecode.append(&mut get_lumi_header(0));
        bytecode.append(&mut vec![0, 0, 8, 0, 0, 1, 5, 0, 4, 0, 1, 2, 5, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run();
        assert_eq!(test_vm.registers[0], 8);
        assert_eq!(test_vm.registers[1], 5);
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 3);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 4;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![6, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![7, 0, 0, 0, 6, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[1] = 6;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![0, 0, 0, 10, 8, 1, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![9, 0, 1, 0, 9, 0, 1, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![10, 0, 1, 0, 10, 0, 1, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 9;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 9;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 9;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 9;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[1] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = false;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![15, 0, 0, 0, 15, 0, 0, 0, 16, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
        test_vm.equal_flag = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 9);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![16, 0, 0, 0, 16, 0, 0, 0, 17, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
        test_vm.equal_flag = false;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1024;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![17, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_inc_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 100;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![18, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 101);
    }

    #[test]
    fn test_dec_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 100;
        let mut bytecode = vec![];
        bytecode.append(&mut vec![19, 0, 0, 0]);
        test_vm.program = bytecode;
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 99);
    }
}
