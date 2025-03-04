use std::fmt;
use std::fmt::Formatter;
use byteorder::{LittleEndian, WriteBytesExt};
use log::{debug, error, info, warn};
use pest::iterators::Pair;
use pest::Parser;
use crate::assembler::{DirectiveType, Token};
use crate::instruction::Opcode;
use crate::assembler_errors::AssemblerError;
use crate::parsers::lumi_asm_parser::{LumiAsmParser, Rule};
use crate::symbols::SymbolTable;

const MAX_I16: i32 = 32767;
const MIN_I16: i32 = -32768;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
  pub(crate) opcode: Option<Token>,
  pub(crate) operand_1: Option<Token>,
  pub(crate) operand_2: Option<Token>,
  pub(crate) operand_3: Option<Token>,
  pub(crate) label: Option<Token>,
  pub(crate) directive: Option<Token>,
}

impl AssemblerInstruction {
  /// Converts this instruction into its binary representation.
  pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
    let mut results = vec![];

    // 1) Write the opcode as 1 byte
    if let Some(Token::Op { code }) = &self.opcode {
      let opcode_byte: u8 = (*code).into(); // from(Opcode)->u8
      results.push(opcode_byte);
    } else {
      // Might be a directive line or label-only line => no bytes
      return results;
    }

    // 2) For each operand, push the correct number of bytes
    for operand in [&self.operand_1, &self.operand_2, &self.operand_3] {
      if let Some(token) = operand {
        Self::extract_operand(token, &mut results, symbols);
      }
    }

    // 3) No forced padding. Return exactly what's needed.
    results
  }
  // pub fn to_bytes(&self, symbol_table: &SymbolTable) -> Vec<u8> {
  //   let mut results = vec![];
  //   if let Some(ref token) = self.opcode {
  //     match token {
  //       Token::Op { code } => {
  //         let b: u8 = (*code).into();
  //         results.push(b);
  //       }
  //       _ => {
  //         error!("Non-opcode token found in opcode field");
  //       }
  //     }
  //   }
  //   // Process operands 1, 2 and 3.
  //   for operand in vec![&self.operand_1, &self.operand_2, &self.operand_3] {
  //     if let Some(token) = operand {
  //       AssemblerInstruction::extract_operand(token, &mut results, symbol_table);
  //     }
  //   }
  //   // Pad the instruction bytes to at least 4 bytes.
  //   // while results.len() < 4 {
  //   //   results.push(0);
  //   // }
  //   results
  // }

  fn extract_operand(token: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
    match token {
      Token::Register { reg_num } => {
        results.push(*reg_num);
      }
      Token::IntegerOperand { value } => {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(*value as i32).unwrap();
        results.extend_from_slice(&wtr);
        // results.push(wtr[0]);
        // results.push(wtr[1]);
      }
      Token::FloatOperand { value } => {
        let mut wtr = vec![];
        wtr.write_f32::<LittleEndian>(*value as f32).unwrap();
        results.extend_from_slice(&wtr);
        // results.push(wtr[0]);
        // results.push(wtr[1]);
      }
      Token::LabelUsage { name } => {
        if let Some(value) = symbols.symbol_value(name) {
          let mut wtr = vec![];
          wtr.write_u32::<LittleEndian>(value).unwrap();
          results.extend_from_slice(&wtr);
        } else {
          error!("Label {} not found in symbol table", name);
        }
      }
      _ => {
        error!("Invalid token found");
      }
    }
  }

  pub fn is_label(&self) -> bool {
    self.label.is_some()
  }

  pub fn is_opcode(&self) -> bool {
    self.opcode.is_some()
  }

  pub fn is_directive(&self) -> bool {
    self.directive.is_some()
  }

  pub fn is_integer_needs_splitting(&self) -> bool {
    if let Some(ref op) = self.opcode {
      match op {
        Token::Op { code } => {
          return match code {
            Opcode::LOAD => {
              if let Some(ref first_half) = self.operand_2 {
                match first_half {
                  Token::IntegerOperand { ref value } => {
                    if *value > MAX_I16 || *value < MIN_I16 {
                      return true;
                    }
                    false
                  }
                  _ => false,
                }
              } else {
                true
              }
            }
            _ => false,
          }
        }
        _ => {}
      }
    }
    false
  }

  pub fn get_integer_value(&self) -> Option<i16> {
    if let Some(ref operand) = self.operand_2 {
      match operand {
        Token::IntegerOperand { ref value } => Some(*value as i16),
        _ => None,
      }
    } else {
      None
    }
  }

  pub fn get_float_value(&self) -> Option<f32> {
    if let Some(ref operand) = self.operand_2 {
      match operand {
        Token::FloatOperand { ref value } => Some(*value as f32),
        _ => None,
      }
    } else {
      None
    }
  }

  pub fn get_register_number(&self) -> Option<u8> {
    match self.operand_1 {
      Some(ref reg_token) => match reg_token {
        Token::Register { ref reg_num } => Some(*reg_num),
        _ => None,
      },
      None => None,
    }
  }

  pub fn set_operand_two(&mut self, token: Token) {
    self.operand_2 = Some(token);
  }

  pub fn set_operand_three(&mut self, token: Token) {
    self.operand_3 = Some(token);
  }

  pub fn has_operands(&self) -> bool {
    self.operand_1.is_some() || self.operand_2.is_some() || self.operand_3.is_some()
  }

  pub fn get_directive_name(&self) -> Option<String> {
    match &self.directive {
      Some(directive) => match directive {
        Token::Directive { directive_type } => Some(format!("{:?}", directive_type)),
        _ => None,
      },
      None => None,
    }
  }

  pub fn get_string_constant(&self) -> Option<String> {
    match &self.operand_1 {
      Some(d) => match d {
        Token::LString { value } => Some(value.to_string()),
        _ => None,
      },
      None => None,
    }
  }

  pub fn get_label_name(&self) -> Option<String> {
    match &self.label {
      Some(l) => match l {
        Token::LabelDeclaration { name } => Some(name.clone()),
        _ => None,
      },
      None => None,
    }
  }

  pub fn get_i32_constant(&self) -> Option<i32> {
    match &self.operand_1 {
      Some(d) => match d {
        Token::IntegerOperand { value } => Some(*value),
        _ => None,
      },
      None => None,
    }
  }

  pub fn get_float_constant(&self) -> Option<f64> {
    match &self.operand_1 {
      Some(d) => match d {
        Token::FloatOperand { value } => Some(*value),
        _ => None,
      },
      None => None,
    }
  }

  /// Converts a Pest parse tree (for a single instruction) into an AssemblerInstruction.
  /// Now it supports both plain directives and directive definitions.
  pub fn from_pair(pair: Pair<Rule>) -> Result<Self, String> {
    let tokens: Vec<_> = pair.into_inner().collect();
    let mut label = None;
    let mut directive = None;
    let mut opcode = None;
    let mut operand_1 = None;
    let mut operand_2 = None;
    let mut operand_3 = None;

    let mut i = 0;

    while i < tokens.len() {
      let t = &tokens[i];
      match t.as_rule() {
        Rule::directive => {
          let text = t.as_str();
          info!("+ Directive: {}", text);
          i += 1;
        }
        Rule::data_declaration => {
          let text = t.as_str();
          info!("+ Data declaration: {}", text);
          i += 1;
        }
        Rule::label_declaration => {
          let text = t.as_str();
          info!("+ Label declaration: {}", text);
          i += 1;
        }
        Rule::identifier => {
          let text = t.as_str();
          info!("+ Identifier: {}", text);
          i += 1;
        }
        Rule::operand => {
          let text = t.as_str();
          info!("+ Operand: {}", text);
          i += 1;
        }
        _ => {
          i += 1;
        }
      }
    }

    i = 0;

    while i < tokens.len() {
      let t = &tokens[i];
      match t.as_rule() {
        Rule::label_declaration => {
          let text = t.as_str();
          let name = text.trim_end_matches(':').to_string();
          label = Some(Token::LabelDeclaration { name });
          i += 1;
        }
        Rule::directive => {
          // Check if the next token is an operand.
          if i + 1 < tokens.len() && tokens[i + 1].as_rule() == Rule::operand {
            // Combine directive and operand into a directive definition.
            let mut dir_text = t.as_str().trim();
            if dir_text.starts_with('.') {
              dir_text = &dir_text[1..];
            }
            let dt = match dir_text.to_lowercase().as_str() {
              "data"    => DirectiveType::Data,
              "text"    => DirectiveType::Text,
              "asciiz"  => DirectiveType::Asciiz,
              "integer" => DirectiveType::Integer,
              "float"   => DirectiveType::Float,
              _         => DirectiveType::Unknown,
            };
            directive = Some(Token::Directive { directive_type: dt });
            // Consume the next token as the operand.
            let op_token = convert_operand(tokens[i + 1].clone())
              .map_err(|e| format!("Error converting operand in directive: {}", e))?;
            operand_1 = Some(op_token);
            i += 2; // skip both tokens
          } else {
            // Otherwise, treat this as a plain directive.
            let mut text = t.as_str().trim();
            if text.starts_with('.') {
              text = &text[1..];
            }
            let dt = match text.to_lowercase().as_str() {
              "data"    => DirectiveType::Data,
              "text"    => DirectiveType::Text,
              "asciiz"  => DirectiveType::Asciiz,
              "integer" => DirectiveType::Integer,
              "float"   => DirectiveType::Float,
              _         => DirectiveType::Unknown,
            };
            directive = Some(Token::Directive { directive_type: dt });
            i += 1;
          }
        }
        Rule::opcode => {
          let text = t.as_str().trim();
          // If the opcode text is numeric, treat it as a constant.
          if let Ok(num) = text.parse::<i32>() {
            operand_1 = Some(Token::IntegerOperand { value: num });
          } else {
            opcode = Some(Token::Op { code: Opcode::from(text) });
          }
          i += 1;
        }
        Rule::operand => {
          debug!("Converting operand: {:?}", t.as_str());
          let op_token = convert_operand(t.clone())
            .map_err(|err| format!("Error converting operand: {}", err))?;
          if operand_1.is_none() {
            operand_1 = Some(op_token);
          } else if operand_2.is_none() {
            operand_2 = Some(op_token);
          } else if operand_3.is_none() {
            operand_3 = Some(op_token);
          } else {
            return Err("Too many operands provided".to_string());
          }
          i += 1;
        }
        _ => {
          warn!("Ignoring unexpected rule: {:?}", t.as_rule());
          i += 1;
        }
      }
    }

    Ok(AssemblerInstruction {
      opcode,
      operand_1,
      operand_2,
      operand_3,
      label,
      directive,
    })
  }
}

pub fn convert_operand(pair: Pair<Rule>) -> Result<Token, String> {
  debug!("convert_operand({:?})", pair.as_rule());
  match pair.as_rule() {
    Rule::register => {
      let s = pair.as_str();
      if s.len() < 1 {
        return Err("Register token too short".to_string());
      }
      let s = &s[1..]; // Skip the '$' prefix.
      let reg_num = s.parse::<u8>().map_err(|err| format!("Invalid register number: {}", err))?;
      Ok(Token::Register { reg_num })
    }
    Rule::int_immediate => {
      let s = pair.as_str();
      debug!("int_immediate: {}", s);
      if s.len() < 1 {
        return Err("Integer immediate token too short".to_string());
      }
      let value = s.parse::<i32>().map_err(|e| format!("Invalid integer immediate: {}", e))?;
      Ok(Token::IntegerOperand { value })
    }
    Rule::float_immediate => {
      let s = pair.as_str();
      if s.len() < 1 {
        return Err("Float immediate token too short".to_string());
      }
      let value = s.parse::<f64>().map_err(|e| format!("Invalid float immediate: {}", e))?;
      Ok(Token::FloatOperand { value })
    }
    Rule::label_usage => {
      let s = pair.as_str();
      if s.len() < 1 {
        return Err("Label usage token too short".to_string());
      }
      let s = &s[1..]; // Skip the '@' prefix.
      Ok(Token::LabelUsage { name: s.parse().unwrap() })
    }
    // In case an operand wraps another operand, drill down.
    Rule::operand => {
      let mut inner = pair.into_inner();
      let inner_pair = inner
        .next()
        .ok_or("Operand rule has no inner pair".to_string())?;
      convert_operand(inner_pair)
    }
    _ => Err(format!("Unexpected operand token: {:?}", pair.as_rule())),
  }
}

impl fmt::Display for AssemblerInstruction {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "(Label: {:?} Opcode: {:?} Directive: {:?} Operand #1: {:?} Operand #2: {:?} Operand #3: {:?})",
      self.label, self.opcode, self.directive, self.operand_1, self.operand_2, self.operand_3
    )
  }
}

/// The new parser function using Pest that converts an input string into an AssemblerInstruction.
/// It delegates the conversion of the Pest parse tree to `AssemblerInstruction::from_pair`.
pub fn parse_instruction(input: &str) -> Result<AssemblerInstruction, AssemblerError> {
  debug!("parse_instruction(\"{}\")", input);
  match LumiAsmParser::parse(Rule::instruction, input) {
    Ok(mut pairs) => {
      if let Some(pair) = pairs.next() {
        AssemblerInstruction::from_pair(pair)
          .map_err(|err| AssemblerError::ParseError { error: err })
      } else {
        Err(AssemblerError::ParseError { error: "No instruction parsed".to_string() })
      }
    }
    Err(e) => Err(AssemblerError::ParseError { error: e.to_string() }),
  }
}
