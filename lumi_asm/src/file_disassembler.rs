use crate::instruction::{Opcode, OperandType};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use colored::Colorize;
use log::debug;
use crate::header_utils::LUMI_HEADER_LENGTH;

#[derive(Debug, Clone, PartialEq)]
pub struct DisassemblyError {
  message: String,
}

pub fn to_hex(value: usize) -> String {
  format!("{:08x}", value)
}

pub fn disassemble(bytecode: &[u8]) -> Result<String, DisassemblyError> {
  let mut output = String::new();

  // Step 1: Validate bytecode length (header + offset + minimum instruction space)
  if bytecode.len() < LUMI_HEADER_LENGTH + 5 {
    return Err(DisassemblyError {
      message: "Bytecode too short to contain header and starting offset".to_string(),
    });
  }

  // Step 2: Calculate Program Counter Start
  const OFFSET_SIZE: usize = 4;

  // Read the RO offset (size of .data)
  let mut rdr = Cursor::new(&bytecode[LUMI_HEADER_LENGTH + 1..LUMI_HEADER_LENGTH + 1 + OFFSET_SIZE]);
  let ro_offset = rdr.read_u32::<LittleEndian>().unwrap() as usize;

  let ro_data_start = LUMI_HEADER_LENGTH + 1 + OFFSET_SIZE; // Start of RO data section
  let ro_data_end = ro_data_start + ro_offset; // End of RO data section
  let mut pc = ro_data_end; // Start of instructions

  debug!("Starting disassembly at PC: {}", pc);
  output.push_str(&format!("; lumi header <0x{:x}-0x{:x}>\n", ro_data_start, ro_data_end));

  // Step 3: Process Instructions
  while pc < bytecode.len() {
    // Ensure valid opcode
    if pc >= bytecode.len() {
      break;
    }

    // Step 3.1: Read Opcode
    let opcode_byte = bytecode[pc];
    debug!("Opcode byte at 0x{:x}: {:?}", pc, opcode_byte);
    pc += 1; // Move past opcode byte

    // Fetch Opcode Metadata
    let opcode = Opcode::from(opcode_byte);
    let metadata = match Opcode::metadata(opcode) {
      Some(m) => m,
      None => {
        return Err(DisassemblyError {
          message: format!("Unknown opcode at PC 0x{:x}: {}", pc - 1, opcode_byte),
        });
      }
    };

    // Append opcode to output
    output.push_str(&format!("0x{:x}: {}\t", pc - 1, metadata.str_symbol));

    // Step 3.2: Decode Operands
    for operand_type in &metadata.operand_types {
      match operand_type {
        OperandType::Register => {
          if pc < bytecode.len() {
            let reg = bytecode[pc];
            output.push_str(&format!("$r{} ", reg));
            pc += 1; // Advance past register byte
          }
        }
        OperandType::FloatRegister => {
          if pc < bytecode.len() {
            let reg = bytecode[pc];
            output.push_str(&format!("$r{} ", reg));
            pc += 1; // Advance past register byte
          }
        }
        OperandType::Address => {
          if pc + 4 <= bytecode.len() { // Ensure enough bytes for address
            let mut rdr = Cursor::new(&bytecode[pc..pc + 4]);
            let address = rdr.read_u32::<LittleEndian>().unwrap();
            output.push_str(&format!("@0x{:x} ", address));
            pc += 4; // Advance by address size (4 bytes)
          }
        }
        OperandType::IntegerImmediate => {
          if pc + 4 <= bytecode.len() { // Ensure enough bytes for integer
            let mut rdr = Cursor::new(&bytecode[pc..pc + 4]);
            let value = rdr.read_i32::<LittleEndian>().unwrap();
            output.push_str(&format!("#{} ", value));
            pc += 4; // Advance by integer size (4 bytes)
          }
        }
        OperandType::FloatImmediate => {
          if pc + 4 <= bytecode.len() { // Ensure enough bytes for integer
            let mut rdr = Cursor::new(&bytecode[pc..pc + 4]);
            let value = rdr.read_f32::<LittleEndian>().unwrap();
            output.push_str(&format!("#{} ", value));
            pc += 4; // Advance by integer size (4 bytes)
          }
        }
        OperandType::Empty => {
          // No operand - do nothing
        }
      }
    }

    // Append newline after each instruction
    output.push_str("\n");
  }

  Ok(output)
}

fn get_byte_color(
  index: usize,
  current_instruction: Option<usize>,
  bytecode: &Vec<u8>,
) -> colored::Color {
  let mut highlight_current = false;
  let mut current_instruction_pos = 0;
  if current_instruction.is_some() {
    highlight_current = true;
    current_instruction_pos = current_instruction.unwrap();
  }

  const OFFSET_SIZE: usize = 4;
  let start_index = LUMI_HEADER_LENGTH + OFFSET_SIZE + 1;
  let mut rdr = Cursor::new(&bytecode[LUMI_HEADER_LENGTH + 1..LUMI_HEADER_LENGTH + 1 + 4]);
  let offset = rdr.read_u32::<LittleEndian>().unwrap() as usize;
  let end_index = start_index + offset - 1;

  // highlight_current = true;
  // current_instruction_pos = LUMI_HEADER_LENGTH + OFFSET_SIZE + 12;

  if index <= LUMI_HEADER_LENGTH {
    // highlight the header
    colored::Color::BrightBlue
  } else if index >= LUMI_HEADER_LENGTH + 1 && index <= LUMI_HEADER_LENGTH + OFFSET_SIZE {
    // highlight the RO offset
    colored::Color::BrightMagenta
  } else if index >= start_index && index <= end_index {
    colored::Color::BrightCyan
  } else if highlight_current && index == current_instruction_pos {
    // highlight current byte that PC is pointing to
    colored::Color::Red
  } else if highlight_current
    && index > current_instruction_pos
    && index <= current_instruction_pos + 3
  {
    // highlight the next 3 bytes (instruction is 32 bits long)
    colored::Color::Yellow
  } else {
    colored::Color::White
  }
}

fn print_ascii_representation(
  line: &[u8],
  line_start_index: usize,
  current_instruction: Option<usize>,
  bytecode: &Vec<u8>,
) {
  print!("  ");
  for (i, &byte) in line.iter().enumerate() {
    let index = line_start_index + i;
    let color = get_byte_color(index, current_instruction, bytecode);

    if byte >= 32 && byte <= 126 {
      print!("{}", (byte as char).to_string().color(color));
    } else {
      print!("{}", ".".color(color));
    }
  }
}

pub fn visualize_program(bytecode: &Vec<u8>, current_instruction: Option<usize>) {
  for (index, &byte) in bytecode.iter().enumerate() {
    // prints the address offset at the start of each line
    if index % 16 == 0 {
      if index != 0 {
        print_ascii_representation(
          &bytecode[index - 16..index],
          index - 16,
          current_instruction,
          bytecode,
        );
        println!();
      }
      print!("{} ", format!("{:08x}", index).green());
    }

    let color = get_byte_color(index, current_instruction, bytecode);

    print!("{}", format!("{:02X}", byte).color(color));

    // group 2 bytes together
    if index % 2 == 1 {
      print!(" ");
    }
  }

  // handle case where program does not end at 16-byte boundary
  let remainder = bytecode.len() % 16;
  if remainder != 0 {
    let padding = (16 - remainder) * 2 + (16 - remainder) / 2 + 1;
    print!("{:padding$}", "", padding = padding);
    print_ascii_representation(
      &bytecode[bytecode.len() - remainder..],
      bytecode.len() - remainder,
      current_instruction,
      bytecode,
    );
  } else {
    print_ascii_representation(
      &bytecode[bytecode.len() - 16..],
      bytecode.len() - 16,
      current_instruction,
      bytecode,
    );
  }
  println!();
}