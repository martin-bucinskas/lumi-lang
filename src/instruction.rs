use crate::vm::VM;

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

pub trait OpcodeExecutor {
    fn execute(&mut self, vm: &mut VM);
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        let uppercase = value.to_uppercase();
        return match uppercase.as_str() {
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "HLT" => Opcode::HLT,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JPMB" => Opcode::JMPB,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "LT" => Opcode::LT,
            "GTE" => Opcode::GTE,
            "LTE" => Opcode::LTE,
            "JMPE" => Opcode::JMPE,
            "DJMPE" => Opcode::DJMPE,
            "ALOC" => Opcode::ALOC,
            "INC" => Opcode::INC,
            "DEC" => Opcode::DEC,
            "NOP" => Opcode::NOP,
            "PRTS" => Opcode::PRTS,
            "LOADF64" => Opcode::LOADF64,
            "ADDF64" => Opcode::ADDF64,
            "SUBF64" => Opcode::SUBF64,
            "MULF64" => Opcode::MULF64,
            "DIVF64" => Opcode::DIVF64,
            "EQF64" => Opcode::EQF64,
            "NEQF64" => Opcode::NEQF64,
            "GTF64" => Opcode::GTF64,
            "GTEF64" => Opcode::GTEF64,
            "LTF64" => Opcode::LTF64,
            "LTEF64" => Opcode::LTEF64,
            "SHL" => Opcode::SHL,
            "SHR" => Opcode::SHR,
            "AND" => Opcode::AND,
            "OR" => Opcode::OR,
            "XOR" => Opcode::XOR,
            "NOT" => Opcode::NOT,
            "LUI" => Opcode::LUI,
            "CLOOP" => Opcode::CLOOP,
            "LOOP" => Opcode::LOOP,
            "LOADM" => Opcode::LOADM,
            "SETM" => Opcode::SETM,
            "PUSH" => Opcode::PUSH,
            "POP" => Opcode::POP,
            "CALL" => Opcode::CALL,
            "RET" => Opcode::RET,
            "DJMP" => Opcode::DJMP,
            "BKPT" => Opcode::BKPT,
            _ => Opcode::IGL,
        };
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        return match value {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JMPE,
            16 => Opcode::DJMPE,
            17 => Opcode::ALOC,
            18 => Opcode::INC,
            19 => Opcode::DEC,
            20 => Opcode::NOP,
            21 => Opcode::PRTS,
            22 => Opcode::LOADF64,
            23 => Opcode::ADDF64,
            24 => Opcode::SUBF64,
            25 => Opcode::MULF64,
            26 => Opcode::DIVF64,
            27 => Opcode::EQF64,
            28 => Opcode::NEQF64,
            29 => Opcode::GTF64,
            30 => Opcode::GTEF64,
            31 => Opcode::LTF64,
            32 => Opcode::LTEF64,
            33 => Opcode::SHL,
            34 => Opcode::SHR,
            35 => Opcode::AND,
            36 => Opcode::OR,
            37 => Opcode::XOR,
            38 => Opcode::NOT,
            39 => Opcode::LUI,
            40 => Opcode::CLOOP,
            41 => Opcode::LOOP,
            42 => Opcode::LOADM,
            43 => Opcode::SETM,
            44 => Opcode::PUSH,
            45 => Opcode::POP,
            46 => Opcode::CALL,
            47 => Opcode::RET,
            48 => Opcode::DJMP,
            49 => Opcode::BKPT,
            _ => Opcode::IGL,
        };
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::LOAD => 0,
            Opcode::ADD => 1,
            Opcode::SUB => 2,
            Opcode::MUL => 3,
            Opcode::DIV => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GT => 11,
            Opcode::LT => 12,
            Opcode::GTE => 13,
            Opcode::LTE => 14,
            Opcode::JMPE => 15,
            Opcode::DJMPE => 16,
            Opcode::ALOC => 17,
            Opcode::INC => 18,
            Opcode::DEC => 19,
            Opcode::NOP => 20,
            Opcode::PRTS => 21,
            Opcode::LOADF64 => 22,
            Opcode::ADDF64 => 23,
            Opcode::SUBF64 => 24,
            Opcode::MULF64 => 25,
            Opcode::DIVF64 => 26,
            Opcode::EQF64 => 27,
            Opcode::NEQF64 => 28,
            Opcode::GTF64 => 29,
            Opcode::GTEF64 => 30,
            Opcode::LTF64 => 31,
            Opcode::LTEF64 => 32,
            Opcode::SHL => 33,
            Opcode::SHR => 34,
            Opcode::AND => 35,
            Opcode::OR => 36,
            Opcode::XOR => 37,
            Opcode::NOT => 38,
            Opcode::LUI => 39,
            Opcode::CLOOP => 40,
            Opcode::LOOP => 41,
            Opcode::LOADM => 42,
            Opcode::SETM => 43,
            Opcode::PUSH => 44,
            Opcode::POP => 45,
            Opcode::CALL => 46,
            Opcode::RET => 47,
            Opcode::DJMP => 48,
            Opcode::BKPT => 49,
            Opcode::IGL => 100,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
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
}
