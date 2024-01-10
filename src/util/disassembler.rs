use crate::instruction::{Opcode, OperandType};
use crate::util::header_utils::LUMI_HEADER_LENGTH;
use crate::util::to_hex;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, Clone, PartialEq)]
pub struct DisassemblyError {
    message: String,
}

pub fn disassemble(bytecode: &[u8]) -> Result<String, DisassemblyError> {
    let mut output = String::new();
    let mut rdr = Cursor::new(&bytecode[LUMI_HEADER_LENGTH + 1..LUMI_HEADER_LENGTH + 1 + 4]);
    let starting_offset = rdr.read_u32::<LittleEndian>().unwrap() as usize;
    let mut pc = LUMI_HEADER_LENGTH + 1 + 4 + starting_offset;
    output.push_str(&*format!("; lumi header <0-{}>\n", pc));

    while pc < bytecode.len() {
        let opcode_byte = bytecode[pc];
        let opcode = Opcode::from(opcode_byte);

        let metadata = opcode.metadata();
        output.push_str(&*format!("0x{}: {}", to_hex(pc), metadata.str_symbol));
        output.push_str(" ");
        let mut offset = 1;
        for operand_type in metadata.operand_types {
            let mut empty = false;
            match operand_type {
                OperandType::Register => output.push_str("$"),
                OperandType::FloatRegister => output.push_str("$"),
                OperandType::Address => output.push_str("@"),
                OperandType::Integer => output.push_str("#"),
                OperandType::Float => output.push_str("#"),
                OperandType::Empty => {
                    output.push_str("");
                    empty = true;
                }
            }
            if pc + offset < bytecode.len() && !empty {
                if operand_type == OperandType::Address {
                    output.push_str(&*format!("0x{}", to_hex(bytecode[pc + offset] as usize)));
                } else {
                    output.push_str(&*format!("{}", bytecode[pc + offset]));
                }
            }
            output.push_str(" ");
            offset = offset + 1;
        }
        output.push_str("\n");
        pc += 4;
    }

    Ok(output)
}
