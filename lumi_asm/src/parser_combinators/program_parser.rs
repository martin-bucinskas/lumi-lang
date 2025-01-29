use log::debug;
use nom::branch::alt;
use nom::combinator::{eof, map, opt};
use nom::error::{context, VerboseError};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;
use crate::core_parsers::directive_parser::parse_directive;
use crate::parser_combinators::instruction_parser::{parse_instruction, AssemblerInstruction};
use crate::symbols::SymbolTable;

#[derive(Debug, PartialEq)]
pub struct Program {
  instructions: Vec<AssemblerInstruction>,
}

impl Program {
  pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
    let mut program = vec![];
    for instruction in &self.instructions {
      program.append(&mut instruction.to_bytes(symbols));
    }
    program
  }

  pub fn get_instructions(&self) -> &Vec<AssemblerInstruction> {
    &self.instructions
  }
}

pub fn parse_program(input: &str) -> IResult<&str, Program, VerboseError<&str>> {
  debug!("parse_program(\"{}\")", input);
  let mut parser = context(
    "Parsing program",
    map(
      terminated(many0(alt((parse_instruction, parse_directive))), opt(eof)),
      |instructions| Program { instructions },
    ),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    debug!("program parser error: {}", err);
  }

  return result;
}

#[cfg(test)]
mod tests {
  use crate::assembler::Token;
  use crate::instruction::Opcode;
  use crate::symbols::SymbolTable;
  use super::*;

  #[test]
  fn test_parse_single_instruction() {
    let result = parse_program("LOAD $0 #100");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_single_instruction_with_newline() {
    let result = parse_program("LOAD $0 #100\n");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_multiple_instructions() {
    let result = parse_program("LOAD $0 #100\nLOAD $1 #200");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_multiple_instructions_with_newline() {
    let result = parse_program("LOAD $0 #100\nLOAD $1 #200\n");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_instruction() {
    let result = parse_program("LOAD $0 #100");
    assert_eq!(
      result,
      Ok((
        "",
        Program {
          instructions: Vec::from([AssemblerInstruction {
            opcode: Some(Token::Op { code: Opcode::LOAD }),
            operand_1: Some(Token::Register { reg_num: 0 }),
            operand_2: Some(Token::IntegerOperand { value: 100 }),
            operand_3: None,
            label: None,
            directive: None,
          }]),
        }
      ))
    )
  }

  #[test]
  fn test_parse_instruction_multispace() {
    let result = parse_program("\t\t LOAD $0 #100\n");
    assert_eq!(
      result,
      Ok((
        "",
        Program {
          instructions: Vec::from([AssemblerInstruction {
            opcode: Some(Token::Op { code: Opcode::LOAD }),
            operand_1: Some(Token::Register { reg_num: 0 }),
            operand_2: Some(Token::IntegerOperand { value: 100 }),
            operand_3: None,
            label: None,
            directive: None,
          }]),
        }
      ))
    )
  }

  #[test]
  fn test_parse_instruction_form_one() {
    let result = parse_program("LOAD $0 #100\nLOAD $1 #200\n");
    assert_eq!(
      result,
      Ok((
        "",
        Program {
          instructions: Vec::from([
            AssemblerInstruction {
              opcode: Some(Token::Op { code: Opcode::LOAD }),
              operand_1: Some(Token::Register { reg_num: 0 }),
              operand_2: Some(Token::IntegerOperand { value: 100 }),
              operand_3: None,
              label: None,
              directive: None,
            },
            AssemblerInstruction {
              opcode: Some(Token::Op { code: Opcode::LOAD }),
              operand_1: Some(Token::Register { reg_num: 1 }),
              operand_2: Some(Token::IntegerOperand { value: 200 }),
              operand_3: None,
              label: None,
              directive: None,
            }
          ]),
        }
      ))
    )
  }

  #[test]
  fn test_program_to_bytes() {
    let result = parse_program("LOAD $1 #500");
    assert_eq!(result.is_ok(), true);
    let (_, program) = result.unwrap();
    let symbols = SymbolTable::new();
    let bytecode = program.to_bytes(&symbols);
    assert_eq!(bytecode.len(), 4);
    assert_eq!(bytecode, [0, 1, 244, 1]);
    println!("{:?}", bytecode);
  }
}
