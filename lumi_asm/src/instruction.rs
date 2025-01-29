use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
  LOAD,
  ADD,
  SUB,
  MUL,
  DIV,
  HLT,
  JMP,
  JMPF,
  JMPB,
  EQ,
  NEQ,
  GT,
  LT,
  GTE,
  LTE,
  JMPE,
  DJMPE,
  ALOC,
  INC,
  DEC,
  NOP,
  PRTS,
  IGL,
  LOADF64,
  ADDF64,
  SUBF64,
  MULF64,
  DIVF64,
  EQF64,
  NEQF64,
  GTF64,
  GTEF64,
  LTF64,
  LTEF64,
  SHL,
  SHR,
  AND,
  OR,
  XOR,
  NOT,
  LUI,
  CLOOP,
  LOOP,
  LOADM,
  SETM,
  PUSH,
  POP,
  CALL,
  RET,
  DJMP,
  BKPT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperandType {
  Register,
  FloatRegister,
  IntegerImmediate,
  FloatImmediate,
  Address,
  Empty,
}

pub struct OpcodeMetadata {
  pub operand_types: [OperandType; 3],
  pub description: &'static str,
  pub str_symbol: &'static str,
  pub bytecode: u8,
}

// pub trait OpcodeExecutor {
//   fn execute(&mut self, vm: &mut VirtualMachine);
// }

static OPCODE_METADATA: &'static [(Opcode, OpcodeMetadata)] = &[
  (Opcode::LOAD, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::IntegerImmediate,
      OperandType::Empty,
    ],
    description: "Loads an integer into a register, use: LOAD $<register> #<value>",
    str_symbol: "LOAD",
    bytecode: 0,
  }),
  (Opcode::ADD, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Adds 2 registers together and saves in another register, use: ADD $<register> $<register> $<register>",
    str_symbol: "ADD",
    bytecode: 1,
  }),
  (Opcode::SUB, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Subtracts 2 registers together and saves in another register, use: SUB $<register> $<register> $<register>",
    str_symbol: "SUB",
    bytecode: 2,
  }),
  (Opcode::MUL, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Multiplies 2 registers together and saves in another register, use: MUL $<register> $<register> $<register>",
    str_symbol: "MUL",
    bytecode: 3,
  }),
  (Opcode::DIV, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Divides 2 registers together and saves in another register, use: DIV $<register> $<register> $<register>",
    str_symbol: "DIV",
    bytecode: 4,
  }),
  (Opcode::HLT, OpcodeMetadata {
    operand_types: [OperandType::Empty, OperandType::Empty, OperandType::Empty],
    description: "Halts the execution, use: HLT",
    str_symbol: "HLT",
    bytecode: 5,
  }),
  (Opcode::JMP, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Jump to a memory address stored in a register, use: JMP $<register>",
    str_symbol: "JMP",
    bytecode: 6,
  }),
  (Opcode::JMPF, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Jump forward by offset stored in a register, use: JMPF $<register>",
    str_symbol: "JMPF",
    bytecode: 7,
  }),
  (Opcode::JMPB, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Jump backward by offset stored in a register, use: JMPB $<register>",
    str_symbol: "JMPB",
    bytecode: 8,
  }),
  (Opcode::EQ, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Compare 2 registers if they're equal, sets the equal_flag to true if they are, use: EQ $<register> $<register>",
    str_symbol: "EQ",
    bytecode: 9,
  }),
  (Opcode::NEQ, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Compare 2 registers if they're not equal, sets the equal_flag to true if the registers are not equal, use: NEQ $<register> $<register>",
    str_symbol: "NEQ",
    bytecode: 10,
  }),
  (Opcode::GT, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Compare 2 registers if register_1 is greater than register_2, sets the equal_flag to true if the registers are not equal, use: GT $<register> $<register>",
    str_symbol: "GT",
    bytecode: 11,
  }),
  (Opcode::LT, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Compare 2 registers if register_1 is less than register_2, sets the equal_flag to true if the registers are not equal, use: LT $<register> $<register>",
    str_symbol: "LT",
    bytecode: 12,
  }),
  (Opcode::GTE, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Compare 2 registers if register_1 is greater than or equal to register_2, sets the equal_flag to true if the registers are not equal, use: GTE $<register> $<register>",
    str_symbol: "GTE",
    bytecode: 13,
  }),
  (Opcode::LTE, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Compare 2 registers if register_1 is less than or equal to register_2, sets the equal_flag to true if the registers are not equal, use: LTE $<register> $<register>",
    str_symbol: "LTE",
    bytecode: 14,
  }),
  (Opcode::JMPE, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Conditional jump if equal_flag is true to a memory address defined in register, use: JMPE $<register>",
    str_symbol: "JMPE",
    bytecode: 15,
  }),
  (Opcode::DJMPE, OpcodeMetadata {
    operand_types: [OperandType::Address, OperandType::Empty, OperandType::Empty],
    description: "Conditional direct jump if equal_flag is true to a memory address, use: JMPE #<integer>",
    str_symbol: "DJMPE",
    bytecode: 16,
  }),
  (Opcode::ALOC, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Allocate memory to the heap with size defined in register, use: ALOC $<register>",
    str_symbol: "ALOC",
    bytecode: 17,
  }),
  (Opcode::INC, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Increments a register by 1, use: INC $<register>",
    str_symbol: "INC",
    bytecode: 18,
  }),
  (Opcode::DEC, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Decrements a register by 1, use: DEC $<register>",
    str_symbol: "DEC",
    bytecode: 19,
  }),
  (Opcode::NOP, OpcodeMetadata {
    operand_types: [OperandType::Empty, OperandType::Empty, OperandType::Empty],
    description: "Performs no operation, use: NOP",
    str_symbol: "NOP",
    bytecode: 20,
  }),
  (Opcode::PRTS, OpcodeMetadata {
    operand_types: [OperandType::Address, OperandType::Empty, OperandType::Empty],
    description: "Print string from heap until a null terminator is reached, heap offset address provided as operand, use: PRTS @label",
    str_symbol: "PRTS",
    bytecode: 21,
  }),
  (Opcode::LOADF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatImmediate,
      OperandType::Empty,
    ],
    description: "Loads a float value to a float register, use: LOADF64 $<float_register> #1.2345",
    str_symbol: "LOADF64",
    bytecode: 22,
  }),
  (Opcode::ADDF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::FloatRegister,
    ],
    description: "Adds 2 float registers together and saves in another float register, use: ADDF64 $<register> $<register> $<register>",
    str_symbol: "ADDF64",
    bytecode: 23,
  }),
  (Opcode::SUBF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::FloatRegister,
    ],
    description: "Subtracts 2 float registers together and saves in another float register, use: SUBF64 $<register> $<register> $<register>",
    str_symbol: "SUBF64",
    bytecode: 24,
  }),
  (Opcode::MULF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::FloatRegister,
    ],
    description: "Multiplies 2 float registers together and saves in another float register, use: MULF64 $<register> $<register> $<register>",
    str_symbol: "MULF64",
    bytecode: 25,
  }),
  (Opcode::DIVF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::FloatRegister,
    ],
    description: "Divides 2 float registers together and saves in another float register, use: DIVF64 $<register> $<register> $<register>",
    str_symbol: "DIVF64",
    bytecode: 26,
  }),
  (Opcode::EQF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::Empty,
    ],
    description: "Compare 2 float registers if they're equal, sets the equal_flag to true if they are, use: EQF64 $<register> $<register>",
    str_symbol: "EQF64",
    bytecode: 27,
  }),
  (Opcode::NEQF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::Empty,
    ],
    description: "Compare 2 float registers if they're not equal, sets the equal_flag to true if they are, use: NEQF64 $<register> $<register>",
    str_symbol: "NEQF64",
    bytecode: 28,
  }),
  (Opcode::GTF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::Empty,
    ],
    description: "Compare 2 float registers if register_1 is greater than register_2, sets the equal_flag to true if they are, use: GTF64 $<register> $<register>",
    str_symbol: "GTF64",
    bytecode: 29,
  }),
  (Opcode::GTEF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::Empty,
    ],
    description: "Compare 2 float registers if register_1 is greater than or equal to register_2, sets the equal_flag to true if they are, use: GTEF64 $<register> $<register>",
    str_symbol: "GTEF64",
    bytecode: 30,
  }),
  (Opcode::LTF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::Empty,
    ],
    description: "Compare 2 float registers if register_1 is less than register_2, sets the equal_flag to true if they are, use: LTF64 $<register> $<register>",
    str_symbol: "LTF64",
    bytecode: 31,
  }),
  (Opcode::LTEF64, OpcodeMetadata {
    operand_types: [
      OperandType::FloatRegister,
      OperandType::FloatRegister,
      OperandType::Empty,
    ],
    description: "Compare 2 float registers if register_1 is less than or equal to register_2, sets the equal_flag to true if they are, use: LTEF64 $<register> $<register>",
    str_symbol: "LTEF64",
    bytecode: 32,
  }),
  (Opcode::SHL, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::IntegerImmediate,
      OperandType::Empty,
    ],
    description: "Shift register value left by integer value, use: SHL $<register> #<integer>",
    str_symbol: "SHL",
    bytecode: 33,
  }),
  (Opcode::SHR, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::IntegerImmediate,
      OperandType::Empty,
    ],
    description: "Shift register value right by integer value, use: SHR $<register> #<integer>",
    str_symbol: "SHR",
    bytecode: 34,
  }),
  (Opcode::AND, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Perform logical AND operation between register_1 and register_2, store output to register_3, use: AND $<register> $<register> $<register>",
    str_symbol: "AND",
    bytecode: 35,
  }),
  (Opcode::OR, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Perform logical OR operation between register_1 and register_2, store output to register_3, use: OR $<register> $<register> $<register>",
    str_symbol: "OR",
    bytecode: 36,
  }),
  (Opcode::XOR, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Register,
    ],
    description: "Perform logical EXCLUSIVE OR operation between register_1 and register_2, store output to register_3, use: XOR $<register> $<register> $<register>",
    str_symbol: "XOR",
    bytecode: 37,
  }),
  (Opcode::NOT, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Perform logical NOT operation on register_1 and store the output to register_2, use: NOT $<register> $<register>",
    str_symbol: "NOT",
    bytecode: 38,
  }),
  (Opcode::LUI, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::IntegerImmediate,
      OperandType::IntegerImmediate,
    ],
    description: "Load upper immediate value to register, use: LUI $<register> #<integer> #<integer>",
    str_symbol: "LUI",
    bytecode: 39,
  }),
  (Opcode::CLOOP, OpcodeMetadata {
    operand_types: [OperandType::IntegerImmediate, OperandType::Empty, OperandType::Empty],
    description: "Create loop setting the loop_count to the value provided, use: CLOOP #<integer>",
    str_symbol: "CLOOP",
    bytecode: 40,
  }),
  (Opcode::LOOP, OpcodeMetadata {
    operand_types: [OperandType::Address, OperandType::Empty, OperandType::Empty],
    description: "Loop to provided address until loop_counter reaches 0, use: LOOP @<address>",
    str_symbol: "LOOP",
    bytecode: 41,
  }),
  (Opcode::LOADM, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Loads heap memory at offset defined in register_1 and stores it to register_2, use: LOADM $<register> $<register>",
    str_symbol: "LOADM",
    bytecode: 42,
  }),
  (Opcode::SETM, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Register,
      OperandType::Empty,
    ],
    description: "Sets heap memory value at offset defined with register_1 with the value of register_2, use: SETM $<register> $<register>",
    str_symbol: "SETM",
    bytecode: 43,
  }),
  (Opcode::PUSH, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Pushes registers value to stack memory, use: PUSH $<register>",
    str_symbol: "PUSH",
    bytecode: 44,
  }),
  (Opcode::POP, OpcodeMetadata {
    operand_types: [
      OperandType::Register,
      OperandType::Empty,
      OperandType::Empty,
    ],
    description: "Pops registers value to stack memory, use: POP $<register>",
    str_symbol: "POP",
    bytecode: 45,
  }),
  (Opcode::CALL, OpcodeMetadata {
    operand_types: [OperandType::Address, OperandType::Empty, OperandType::Empty],
    description: "Calls a subroutine and creates a return destination, gets the address destination to jump to, pushes the return address to stack, use: CALL @<address>",
    str_symbol: "CALL",
    bytecode: 46,
  }),
  (Opcode::RET, OpcodeMetadata {
    operand_types: [OperandType::Empty, OperandType::Empty, OperandType::Empty],
    description: "Returns from a subroutine, pops the return address, use: RET",
    str_symbol: "RET",
    bytecode: 47,
  }),
  (Opcode::DJMP, OpcodeMetadata {
    operand_types: [OperandType::Address, OperandType::Empty, OperandType::Empty],
    description: "Direct jump to a memory address, use DJMP @<address>",
    str_symbol: "DJMP",
    bytecode: 48,
  }),
  (Opcode::BKPT, OpcodeMetadata {
    operand_types: [OperandType::Empty, OperandType::Empty, OperandType::Empty],
    description: "Creates a breakpoint in the program for debugging, use: BKPT",
    str_symbol: "BKPT",
    bytecode: 49,
  }),
  (Opcode::IGL, OpcodeMetadata {
    operand_types: [OperandType::Empty, OperandType::Empty, OperandType::Empty],
    description: "Invalid opcode, should never be used directly, use: IGL",
    str_symbol: "IGL",
    bytecode: 100,
  })
];

lazy_static! {
  static ref BYTE_TO_OPCODE_MAP: HashMap<u8, Opcode> = {
    let mut map = HashMap::new();
    for &(opcode, ref metadata) in OPCODE_METADATA.iter() {
      map.insert(metadata.bytecode, opcode);
    }
    map
  };
  
  static ref STR_TO_OPCODE_MAP: HashMap<&'static str, Opcode> = {
    let mut map = HashMap::new();
    for &(opcode, ref metadata) in OPCODE_METADATA.iter() {
      map.insert(metadata.str_symbol, opcode);
    }
    map
  };
}

impl Opcode {
  pub fn from_byte(byte: u8) -> Option<Self> {
    BYTE_TO_OPCODE_MAP.get(&byte).copied()
  }
  
  pub fn from_str(s: &str) -> Option<Self> {
    STR_TO_OPCODE_MAP.get(s.to_uppercase().as_str()).copied()
  }

  pub fn metadata(opcode: Opcode) -> Option<&'static OpcodeMetadata> {
    OPCODE_METADATA
      .iter()
      .find(|(op, _)| *op == opcode)
      .map(|(_, metadata)| metadata)
  }
}

impl From<&str> for Opcode {
  fn from(value: &str) -> Self {
    let uppercase = value.to_uppercase();
    Opcode::from_str(uppercase.as_str())
      .or(Some(Opcode::IGL))
      .expect("failed to convert str to opcode")
  }
}

impl From<u8> for Opcode {
  fn from(value: u8) -> Self {
    Opcode::from_byte(value)
      .or(Some(Opcode::IGL))
      .expect("failed to convert byte to opcode")
  }
}

impl From<Opcode> for u8 {
  fn from(op: Opcode) -> Self {
    OPCODE_METADATA
      .iter()
      .find(|(opcode, _)| *opcode == op)
      .map(|(_, metadata)| metadata.bytecode)
      .unwrap_or(Opcode::IGL as u8)
  }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
  opcode: Opcode,
}

impl Instruction {
  pub fn new(opcode: Opcode) -> Instruction {
    Instruction {
      opcode
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_create_hlt() {
    let opcode = Opcode::HLT;
    assert_eq!(opcode, Opcode::HLT);
  }
  
  #[test]
  fn test_create_instruction() {
    let instruction = Instruction::new(Opcode::HLT);
    assert_eq!(instruction.opcode, Opcode::HLT);
  }
  
  #[test]
  fn test_str_to_opcode() {
    let opcode = Opcode::from("load");
    assert_eq!(opcode, Opcode::LOAD);
    
    let opcode = Opcode::from("illegal");
    assert_eq!(opcode, Opcode::IGL);
  }
  
  #[test]
  fn test_opcode_metadata_lookup() {
    let metadata = OPCODE_METADATA
      .iter()
      .find(|(opcode, _)| *opcode == Opcode::LOAD)
      .map(|(_, metadata)| metadata);
    
    assert!(metadata.is_some(), "Metadata for Opcode::LOAD should exist");
    
    let metadata = metadata.unwrap();
    assert_eq!(metadata.str_symbol, "LOAD");
    assert_eq!(metadata.bytecode, 0);
    assert_eq!(metadata.operand_types[0], OperandType::Register);
  }
}