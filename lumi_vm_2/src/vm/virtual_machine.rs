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
    }
  }
  
  pub fn load_extensions(path: &str) -> Result<Box<dyn LumiVmPlugin>, Box<dyn Error>> {
    unsafe {
      let lib = Library::new(path)?;
      let create_extension_fn: Symbol<extern "C" fn() -> Box<dyn LumiVmPlugin>> =
        lib.get(b"create_extension")?;
      Ok(create_extension_fn())
    }
  }

  pub fn run(&mut self) -> Vec<VMEvent> {
    self.events.push(VMEvent {
      event_type: VMEventType::Start,
      at: Utc::now(),
      application_id: self.vm_id,
      message: None,
    });
    
    let mut extensions: Vec<Box<dyn LumiVmPlugin>> = vec![];
    
    // todo: load extensions from a directory
    if let Ok(ext) = self.load_extensions("extensions") {
      extensions.push(ext);
    }
    
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
    // todo: use a lookup table for instruction decoding
    match opcode {
      Opcode::HLT => {
        info!("HLT encountered, exiting.");
        return ExecutionStatus::Done(0);
      }
      Opcode::BKPT => {
        self.next_8_bits();
        self.next_8_bits();
        self.next_8_bits();
        return ExecutionStatus::BreakpointHit;
      }
      Opcode::LOAD => self.memory_execute_load(),
      Opcode::LOADF64 => self.memory_execute_load_f64(),
      Opcode::ALOC => self.memory_execute_allocate(),
      Opcode::LUI => self.memory_execute_load_upper_immediate(),
      Opcode::SETM => self.memory_execute_set_memory(),
      Opcode::LOADM => self.memory_execute_load_memory(),
      Opcode::PUSH => self.memory_execute_push_to_stack(),
      Opcode::POP => self.memory_execute_pop_from_stack(),

      Opcode::ADD => self.arithmetic_execute_add(),
      Opcode::SUB => self.arithmetic_execute_sub(),
      Opcode::MUL => self.arithmetic_execute_mul(),
      Opcode::DIV => self.arithmetic_execute_div(),

      Opcode::ADDF64 => self.arithmetic_execute_add_f64(),
      Opcode::SUBF64 => self.arithmetic_execute_sub_f64(),
      Opcode::MULF64 => self.arithmetic_execute_mul_f64(),
      Opcode::DIVF64 => self.arithmetic_execute_div_f64(),

      Opcode::INC => self.arithmetic_execute_increment(),
      Opcode::DEC => self.arithmetic_execute_decrement(),

      Opcode::JMP => self.control_execute_jump(),
      Opcode::JMPF => self.control_execute_jump_forward(),
      Opcode::JMPB => self.control_execute_jump_backward(),
      Opcode::JMPE => self.control_execute_jump_if_equal(),
      Opcode::DJMP => self.control_execute_direct_jump(),
      Opcode::DJMPE => self.control_execute_direct_jump_if_equal(),
      Opcode::LOOP => self.control_execute_loop(),
      Opcode::CLOOP => self.control_execute_create_loop(),

      Opcode::EQ => self.comparison_execute_equal(),
      Opcode::NEQ => self.comparison_execute_not_equal(),
      Opcode::GT => self.comparison_execute_greater_than(),
      Opcode::LT => self.comparison_execute_less_than(),
      Opcode::GTE => self.comparison_execute_greater_than_or_equal(),
      Opcode::LTE => self.comparison_execute_less_than_or_equal(),

      Opcode::EQF64 => self.comparison_execute_equal_f64(),
      Opcode::NEQF64 => self.comparison_execute_not_equal_f64(),
      Opcode::GTF64 => self.comparison_execute_greater_than_f64(),
      Opcode::LTF64 => self.comparison_execute_less_than_f64(),
      Opcode::GTEF64 => self.comparison_execute_greater_than_or_equal_f64(),
      Opcode::LTEF64 => self.comparison_execute_less_than_or_equal_f64(),

      Opcode::SHL => self.bitwise_execute_shift_left(),
      Opcode::SHR => self.bitwise_execute_shift_right(),

      Opcode::AND => self.logical_execute_and(),
      Opcode::OR => self.logical_execute_or(),
      Opcode::XOR => self.logical_execute_xor(),
      Opcode::NOT => self.logical_execute_not(),


      Opcode::PRTS => self.system_execute_print_string(),
      Opcode::CALL => self.system_execute_call(),
      Opcode::RET => self.system_execute_return(),

      Opcode::NOP => {
        self.next_8_bits();
        self.next_8_bits();
        self.next_8_bits();
      }
      Opcode::IGL => {
        error!("Illegal instruction encountered.");
        return ExecutionStatus::Done(1);
      }
    }
    
    
    ExecutionStatus::Continue
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
