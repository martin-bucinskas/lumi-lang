use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::{DateTime, Utc};
use libloading::{Library, Symbol};
use log::{debug, error, info};
use uuid::Uuid;
use lumi_asm::instruction::Opcode;
use lumi_asm::header_utils::{verify_header, LUMI_HEADER_LENGTH};
use lumi_vm_sdk::{LumiVmContext, LumiVmPlugin};
use crate::vm::extensions::load_extensions;
use crate::vm::operations::InstructionHandler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMEventType {
  Start,
  Info,
  GracefulShutdown { exit_code: u32 },
  ForcefulShutdown { exit_code: u32 },
  Crash { exit_code: u32 },
}

pub enum ExecutionStatus {
  Continue,
  BreakpointHit,
  Crash(u32),
  Done(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMEvent {
  pub event_type: VMEventType,
  at: DateTime<Utc>,
  application_id: Uuid,
  message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WatchType {
  Memory(usize),
  Register(usize),
  FloatRegister(usize),
}

#[derive(Debug, Clone)]
pub struct WatchVariable {
  pub(crate) watch_type: WatchType,
  pub(crate) last_value: f32,
}

impl VMEventType {
  pub fn stop_code(&self) -> u32 {
    match self {
      VMEventType::Start => 0,
      VMEventType::Info => 0,
      VMEventType::GracefulShutdown { exit_code } => *exit_code,
      VMEventType::ForcefulShutdown { exit_code } => *exit_code,
      VMEventType::Crash { exit_code } => *exit_code,
    }
  }
}

pub struct VirtualMachine {
  pub vm_id: Uuid,
  pub logical_cores: usize,
  pub registers: [i32; 32],
  pub float_registers: [f64; 32],
  pub pc: usize,
  pub program: Vec<u8>,
  pub heap: Vec<u8>,
  pub stack: Vec<i32>,
  pub remainder: u32,
  pub equal_flag: bool,
  pub events: Vec<VMEvent>,
  pub ro_data: Vec<u8>,
  pub loop_counter: usize,
  pub sp: usize,
  pub bp: usize,
  pub watch_variables: HashMap<WatchType, WatchVariable>,
  pub instruction_table: HashMap<Opcode, InstructionHandler>,
}

impl VirtualMachine {
  pub fn new() -> Self {
    VirtualMachine {
      vm_id: Uuid::new_v4(),
      logical_cores: num_cpus::get(),
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
      instruction_table: HashMap::new(),
    }
  }
  
  pub fn initialize() -> Self {
    let mut vm = VirtualMachine::new();
    vm.initialize_instruction_table();
    vm
  }
  
  fn initialize_instruction_table(&mut self) {
    self.instruction_table.insert(Opcode::LOAD, VirtualMachine::memory_execute_load);
    self.instruction_table.insert(Opcode::LOADF64, VirtualMachine::memory_execute_load_f64);
    self.instruction_table.insert(Opcode::ALOC, VirtualMachine::memory_execute_allocate);
    self.instruction_table.insert(Opcode::LUI, VirtualMachine::memory_execute_load_upper_immediate);
    self.instruction_table.insert(Opcode::SETM, VirtualMachine::memory_execute_set_memory);
    self.instruction_table.insert(Opcode::LOADM, VirtualMachine::memory_execute_load_memory);
    self.instruction_table.insert(Opcode::PUSH, VirtualMachine::memory_execute_push_to_stack);
    self.instruction_table.insert(Opcode::POP, VirtualMachine::memory_execute_pop_from_stack);
    
    self.instruction_table.insert(Opcode::ADD, VirtualMachine::arithmetic_execute_add);
    self.instruction_table.insert(Opcode::SUB, VirtualMachine::arithmetic_execute_sub);
    self.instruction_table.insert(Opcode::MUL, VirtualMachine::arithmetic_execute_mul);
    self.instruction_table.insert(Opcode::DIV, VirtualMachine::arithmetic_execute_div);
    
    self.instruction_table.insert(Opcode::ADDF64, VirtualMachine::arithmetic_execute_add_f64);
    self.instruction_table.insert(Opcode::SUBF64, VirtualMachine::arithmetic_execute_sub_f64);
    self.instruction_table.insert(Opcode::MULF64, VirtualMachine::arithmetic_execute_mul_f64);
    self.instruction_table.insert(Opcode::DIVF64, VirtualMachine::arithmetic_execute_div_f64);
    
    self.instruction_table.insert(Opcode::INC, VirtualMachine::arithmetic_execute_increment);
    self.instruction_table.insert(Opcode::DEC, VirtualMachine::arithmetic_execute_decrement);
    
    self.instruction_table.insert(Opcode::JMP, VirtualMachine::control_execute_jump);
    self.instruction_table.insert(Opcode::JMPF, VirtualMachine::control_execute_jump_forward);
    self.instruction_table.insert(Opcode::JMPB, VirtualMachine::control_execute_jump_backward);
    self.instruction_table.insert(Opcode::JMPE, VirtualMachine::control_execute_jump_if_equal);
    self.instruction_table.insert(Opcode::DJMP, VirtualMachine::control_execute_direct_jump);
    self.instruction_table.insert(Opcode::DJMPE, VirtualMachine::control_execute_direct_jump_if_equal);
    self.instruction_table.insert(Opcode::LOOP, VirtualMachine::control_execute_loop);
    self.instruction_table.insert(Opcode::CLOOP, VirtualMachine::control_execute_create_loop);
    
    self.instruction_table.insert(Opcode::EQ, VirtualMachine::comparison_execute_equal);
    self.instruction_table.insert(Opcode::NEQ, VirtualMachine::comparison_execute_not_equal);
    self.instruction_table.insert(Opcode::GT, VirtualMachine::comparison_execute_greater_than);
    self.instruction_table.insert(Opcode::LT, VirtualMachine::comparison_execute_less_than);
    self.instruction_table.insert(Opcode::GTE, VirtualMachine::comparison_execute_greater_than_or_equal);
    self.instruction_table.insert(Opcode::LTE, VirtualMachine::comparison_execute_less_than_or_equal);
    
    self.instruction_table.insert(Opcode::EQF64, VirtualMachine::comparison_execute_equal_f64);
    self.instruction_table.insert(Opcode::NEQF64, VirtualMachine::comparison_execute_not_equal_f64);
    self.instruction_table.insert(Opcode::GTF64, VirtualMachine::comparison_execute_greater_than_f64);
    self.instruction_table.insert(Opcode::LTF64, VirtualMachine::comparison_execute_less_than_f64);
    self.instruction_table.insert(Opcode::GTEF64, VirtualMachine::comparison_execute_greater_than_or_equal_f64);
    self.instruction_table.insert(Opcode::LTEF64, VirtualMachine::comparison_execute_less_than_or_equal_f64);
    
    self.instruction_table.insert(Opcode::SHL, VirtualMachine::bitwise_execute_shift_left);
    self.instruction_table.insert(Opcode::SHR, VirtualMachine::bitwise_execute_shift_right);
    
    self.instruction_table.insert(Opcode::AND, VirtualMachine::logical_execute_and);
    self.instruction_table.insert(Opcode::OR, VirtualMachine::logical_execute_or);
    self.instruction_table.insert(Opcode::XOR, VirtualMachine::logical_execute_xor);
    self.instruction_table.insert(Opcode::NOT, VirtualMachine::logical_execute_not);
    
    self.instruction_table.insert(Opcode::PRTS, VirtualMachine::system_execute_print_string);
    self.instruction_table.insert(Opcode::CALL, VirtualMachine::system_execute_call);
    self.instruction_table.insert(Opcode::RET, VirtualMachine::system_execute_return);
    
    self.instruction_table.insert(Opcode::NOP, VirtualMachine::system_no_operation);
    self.instruction_table.insert(Opcode::HLT, VirtualMachine::system_halt);
    self.instruction_table.insert(Opcode::IGL, VirtualMachine::system_illegal_instruction);
  }

  fn dump_state(&self) {
    info!("Registers: {:?}", self.registers);
    info!("Float Registers: {:?}", self.float_registers);
    info!("PC: {}", self.pc);
    info!("SP: {}", self.sp);
    info!("BP: {}", self.bp);
    info!("Stack: {:?}", self.stack);
    info!("Heap: {:?}", self.heap);
    info!("RO Data: {:?}", self.ro_data);
  }

  // TODO: implement this
  fn detect_changes(&mut self) {
    for (watch_key, watch_var) in self.watch_variables.iter_mut() {
      let current_value = match watch_key {
        WatchType::Memory(addr) => self.heap[*addr] as f32,
        WatchType::Register(index) => self.registers[*index] as f32,
        WatchType::FloatRegister(index) => self.float_registers[*index] as f32,
      };

      if (current_value - watch_var.last_value).abs() > f32::EPSILON {
        info!("Watched variable changed: {:?}", watch_var);
        watch_var.last_value = current_value;
      }
    }
  }

  pub fn run(&mut self) -> Vec<VMEvent> {
    self.events.push(VMEvent {
      event_type: VMEventType::Start,
      at: Utc::now(),
      application_id: self.vm_id,
      message: None,
    });
    
    let mut extensions: Vec<Box<dyn LumiVmPlugin>> = load_extensions("./extensions");
    
    for ext in &mut extensions {
      let context = LumiVmContext {
        vm_name: self.vm_id.to_string(),
        register_hook: &|_name, _hook| {
          
        },
      };
      ext.on_load().unwrap();
    }

    if !verify_header(&self.program) {
      self.events.push(VMEvent {
        event_type: VMEventType::Crash { exit_code: 1 },
        at: Utc::now(),
        application_id: self.vm_id,
        message: Some("Not a LUMI header, skipping execution.".to_string()),
      });
      error!("Not a LUMI header, skipping execution.");
      return self.events.clone();
    }

    // move PC past header
    self.pc = LUMI_HEADER_LENGTH + 1 + 4 + self.get_starting_offset();
    debug!("code start: {}", self.pc);

    self.read_ro_data();

    let mut is_done = None;
    let mut in_step_mode = false;
    while is_done.is_none() {
      match self.execute_instruction() {
        ExecutionStatus::Continue => {
          if in_step_mode {
            // in_step_mode = self.system_execute_breakpoint();
          }
        }
        ExecutionStatus::BreakpointHit => {
          // in_step_mode = self.system_execute_breakpoint();
        }
        ExecutionStatus::Crash(code) => {
          // TODO: log crash event
          is_done = Some(code);
        }
        ExecutionStatus::Done(code) => {
          is_done = Some(code);
        }
      }
    }

    self.events.push(VMEvent {
      event_type: VMEventType::GracefulShutdown { exit_code: is_done.unwrap() },
      at: Utc::now(),
      application_id: self.vm_id,
      message: None,
    });

    self.events.clone()
  }

  /// Run the VM for one instruction.
  pub fn run_once(&mut self) {
    self.execute_instruction();
  }

  /// Execute the next instruction in the program.
  pub fn execute_instruction(&mut self) -> ExecutionStatus {
    if self.pc >= self.program.len() {
      return ExecutionStatus::Done(1);
    }
    
    // check for watch variable changes
    for watch_var in self.watch_variables.values_mut() {
      let current_value = match watch_var.watch_type {
        WatchType::Memory(addr) => self.heap[addr] as f32,
        WatchType::Register(index) => self.registers[index] as f32,
        WatchType::FloatRegister(index) => self.float_registers[index] as f32,
      };
      
      if current_value != watch_var.last_value {
        info!("Watched variable changed: {:?}", watch_var);
        watch_var.last_value = current_value;
      }
    }

    let opcode = self.decode_opcode();
    if let Some(handler) = self.instruction_table.get(&opcode) {
      return handler(self);
    } else {
      error!("Illegal instruction: {:?}", opcode);
      return ExecutionStatus::Done(1);
    }
  }

  /// Decode the current opcode from the program and increment the program counter.
  pub fn decode_opcode(&mut self) -> Opcode {
    let byte = self.program[self.pc];
    let opcode = Opcode::from(byte);
    self.pc += 1;
    opcode
  }
  
  /// Read the next 8 bits from the program and increment the program counter.
  pub fn next_8_bits(&mut self) -> u8 {
    let result = self.program[self.pc];
    self.pc += 1;
    result
  }
  
  
  /// Read the next 16 bits from the program and increment the program counter.
  /// Uses little-endian format.
  pub fn next_16_bits(&mut self) -> u16 {
    let result = (self.program[self.pc] as u16) | ((self.program[self.pc + 1] as u16) << 8);
    self.pc += 2;
    result
  }
  
  /// Read the next 24 bits from the program and increment the program counter.
  /// Uses little-endian format.
  pub fn next_24_bits(&mut self) -> u32 {
    let result = (self.program[self.pc] as u32)
      | ((self.program[self.pc + 1] as u32) << 8)
      | ((self.program[self.pc + 2] as u32) << 16);
    self.pc += 3;
    result
  }

  /// Get the programs starting offset.
  pub fn get_starting_offset(&self) -> usize {
    let mut rdr =
      Cursor::new(&self.program[LUMI_HEADER_LENGTH + 1.. LUMI_HEADER_LENGTH + 1 + 4]);
    rdr.read_u32::<LittleEndian>().unwrap() as usize
  }


  /// Read the read-only data from the program.
  pub fn read_ro_data(&mut self) {
    const OFFSET_SIZE: usize = 4;

    let start_index = LUMI_HEADER_LENGTH + OFFSET_SIZE + 1;
    let end_index = start_index + self.get_starting_offset();
    debug!("Reading read-only data from {} to {}", start_index, end_index);
    self.ro_data = self.program[start_index..end_index].to_vec();
  }
}
