use std::str::FromStr;
use log::{debug, error};
use nom::branch::alt;
use nom::combinator::{eof, map, opt};
use nom::error::{context, VerboseError};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::IResult;
use pest::iterators::Pairs;
use crate::assembler::{DirectiveType, Token};
use crate::assembler_errors::AssemblerError;
use crate::parsers::assembler_instruction::{convert_operand, AssemblerInstruction};
use crate::parsers::lumi_asm_parser::Rule;
use crate::symbols::SymbolTable;

#[derive(Debug, PartialEq)]
pub struct Program {
  instructions: Vec<AssemblerInstruction>,
}

impl Program {
  pub fn new(instructions: Vec<AssemblerInstruction>) -> Self {
    Self { instructions }
  }
  
  pub fn get_instructions(&self) -> &Vec<AssemblerInstruction> {
    &self.instructions
  }

  pub fn from_pairs(mut pairs: Pairs<Rule>) -> Result<Self, AssemblerError> {
    // The parse result is a top-level pair with rule `program`.
    // Get the single top-level pair.
    let program_pair = pairs
      .next()
      .ok_or(AssemblerError::ParseError { error: "No top-level program found".to_string() })?;

    let mut instructions = Vec::new();

    // Iterate over the inner pairs of the top-level program.
    for pair in program_pair.into_inner() {
      
      let mut label: Option<Token> = None;
      let mut directive: Option<Token> = None;
      let mut opcode: Option<Token> = None;
      let mut operand_1: Option<Token> = None;
      let mut operand_2: Option<Token> = None;
      let mut operand_3: Option<Token> = None;
      
      match pair.as_rule() {
        Rule::directive => {
          let directive_type = pair.as_str();
          directive = Some(Token::Directive { directive_type: DirectiveType::from_str(directive_type).unwrap() });
          debug!("Found directive - {}: {:?}", directive_type, directive);
        },
        Rule::data_declaration => {
          let mut inner_pairs = pair.into_inner();
          while let Some(inner_pair) = inner_pairs.next() {
            if inner_pair.as_rule() == Rule::label_declaration {
              let identifier_pair = inner_pair.into_inner().next().unwrap();
              let label_name = identifier_pair.as_str();
              label = Some(Token::LabelDeclaration { name: label_name.to_string() });
              debug!("Found label: {:?}", label);
            } else if inner_pair.as_rule() == Rule::directive {
              let directive_type = inner_pair.as_str();
              directive = Some(Token::Directive { directive_type: DirectiveType::from_str(directive_type).unwrap() });
              debug!("Found directive - {}: {:?}", directive_type, directive);
            } else if inner_pair.as_rule() == Rule::operand {
              debug!("Found operand: {:?}", inner_pair);
              let operand_pair = inner_pair.into_inner().next().unwrap();
              debug!("Found operand pair: {:?}", operand_pair);
              let operand = match operand_pair.as_rule() {
                Rule::register => convert_operand(operand_pair),
                Rule::int_immediate => convert_operand(operand_pair),
                Rule::float_immediate => convert_operand(operand_pair),
                Rule::string_immediate => convert_operand(operand_pair),
                Rule::label_usage => convert_operand(operand_pair),
                _ => {
                  error!("Unexpected rule for operand: {:?}", operand_pair.as_rule());
                  return Result::Err(AssemblerError::ParseError { error: "Unexpected rule for operand".to_string() });
                }
              };
              
              if operand.is_ok() {
                let operand_token = operand.unwrap();
                if operand_1.is_none() {
                  operand_1 = Some(operand_token);
                  debug!("Found operand 1: {:?}", operand_1);
                } else if operand_2.is_none() {
                  operand_2 = Some(operand_token);
                  debug!("Found operand 2: {:?}", operand_2);
                } else if operand_3.is_none() {
                  operand_3 = Some(operand_token);
                  debug!("Found operand 3: {:?}", operand_3);
                } else {
                  error!("Too many operands found in data declaration");
                }
              } else {
                error!("Error converting operand: {:?}", operand);
              }
            }
          }
        }
        Rule::label_declaration => {
          if let Some(identifier_pair) = pair.into_inner().next() {
            if identifier_pair.as_rule() == Rule::identifier {
              let label_name = identifier_pair.as_str();
              label = Some(Token::LabelDeclaration { name: label_name.to_string() });
              debug!("Found label: {:?}", label);
            } else {
              error!("Expected an identifier for label declaration but found: {:?} - `{:?}`", identifier_pair.as_rule(), identifier_pair.as_str())
            }
          } else {
            error!("No inner rules found in label_declaration");
          }
        }
        Rule::instruction => {
          // Convert each line into an instruction.
          let instruction = AssemblerInstruction::from_pair(pair)
            .map_err(|e| AssemblerError::ParseError { error: e })?;
          debug!("instruction: {:?}", instruction);
          instructions.push(instruction);
        }
        _ => {
          // Skip or log any unexpected rule.
          debug!("Skipping unexpected rule: {:?}", pair.as_rule());
        }
      }
      
      let instruction = AssemblerInstruction {
        label,
        directive,
        opcode,
        operand_1,
        operand_2,
        operand_3,
      };
      instructions.push(instruction);
    }

    Ok(Program::new(instructions))
  }
  
  pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
    let mut program = vec![];
    for instruction in &self.instructions {
      program.append(&mut instruction.to_bytes(symbols));
    }
    program
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::assembler::Token;
//   use crate::instruction::Opcode;
//   use crate::symbols::SymbolTable;
//   use super::*;
// 
//   #[test]
//   fn test_parse_single_instruction() {
//     let result = parse_program("LOAD $0 #100");
//     assert_eq!(result.is_ok(), true);
//   }
// 
//   #[test]
//   fn test_parse_single_instruction_with_newline() {
//     let result = parse_program("LOAD $0 #100\n");
//     assert_eq!(result.is_ok(), true);
//   }
// 
//   #[test]
//   fn test_parse_multiple_instructions() {
//     let result = parse_program("LOAD $0 #100\nLOAD $1 #200");
//     assert_eq!(result.is_ok(), true);
//   }
// 
//   #[test]
//   fn test_parse_multiple_instructions_with_newline() {
//     let result = parse_program("LOAD $0 #100\nLOAD $1 #200\n");
//     assert_eq!(result.is_ok(), true);
//   }
// 
//   #[test]
//   fn test_parse_instruction() {
//     let result = parse_program("LOAD $0 #100");
//     assert_eq!(
//       result,
//       Ok((
//         "",
//         Program {
//           instructions: Vec::from([AssemblerInstruction {
//             opcode: Some(Token::Op { code: Opcode::LOAD }),
//             operand_1: Some(Token::Register { reg_num: 0 }),
//             operand_2: Some(Token::IntegerOperand { value: 100 }),
//             operand_3: None,
//             label: None,
//             directive: None,
//           }]),
//         }
//       ))
//     )
//   }
// 
//   #[test]
//   fn test_parse_instruction_multispace() {
//     let result = parse_program("\t\t LOAD $0 #100\n");
//     assert_eq!(
//       result,
//       Ok((
//         "",
//         Program {
//           instructions: Vec::from([AssemblerInstruction {
//             opcode: Some(Token::Op { code: Opcode::LOAD }),
//             operand_1: Some(Token::Register { reg_num: 0 }),
//             operand_2: Some(Token::IntegerOperand { value: 100 }),
//             operand_3: None,
//             label: None,
//             directive: None,
//           }]),
//         }
//       ))
//     )
//   }
// 
//   #[test]
//   fn test_parse_instruction_form_one() {
//     let result = parse_program("LOAD $0 #100\nLOAD $1 #200\n");
//     assert_eq!(
//       result,
//       Ok((
//         "",
//         Program {
//           instructions: Vec::from([
//             AssemblerInstruction {
//               opcode: Some(Token::Op { code: Opcode::LOAD }),
//               operand_1: Some(Token::Register { reg_num: 0 }),
//               operand_2: Some(Token::IntegerOperand { value: 100 }),
//               operand_3: None,
//               label: None,
//               directive: None,
//             },
//             AssemblerInstruction {
//               opcode: Some(Token::Op { code: Opcode::LOAD }),
//               operand_1: Some(Token::Register { reg_num: 1 }),
//               operand_2: Some(Token::IntegerOperand { value: 200 }),
//               operand_3: None,
//               label: None,
//               directive: None,
//             }
//           ]),
//         }
//       ))
//     )
//   }
// 
//   #[test]
//   fn test_program_to_bytes() {
//     let result = parse_program("LOAD $1 #500");
//     assert_eq!(result.is_ok(), true);
//     let (_, program) = result.unwrap();
//     let symbols = SymbolTable::new();
//     let bytecode = program.to_bytes(&symbols);
//     assert_eq!(bytecode.len(), 4);
//     assert_eq!(bytecode, [0, 1, 244, 1]);
//     println!("{:?}", bytecode);
//   }
// }
