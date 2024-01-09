use crate::instruction::Opcode;

pub struct DisassemblyError {
    message: String,
}

pub fn disassemble(bytecode: &[u8]) -> Result<String, DisassemblyError> {
    let mut output = String::new();
    let mut pc = 0;

    while pc < bytecode.len() {
        let opcode_byte = bytecode[pc];
        let opcode = Opcode::from(opcode_byte);
        let operand_1 = bytecode[pc + 1];
        let operand_2 = bytecode[pc + 2];
        let operand_3 = bytecode[pc + 3];
        pc += 4;
    }

    Ok(output)
}
