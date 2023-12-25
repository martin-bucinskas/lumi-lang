use crate::assembler::comment_parsers::parse_comment;
use crate::assembler::label_parsers::parse_label_declaration;
use crate::assembler::opcode_parsers::parse_opcode;
use crate::assembler::operand_parsers::parse_operand;
use crate::assembler::separator_parsers::parse_separator;
use crate::assembler::{SymbolTable, Token};
use crate::instruction::Opcode;
use byteorder::{LittleEndian, WriteBytesExt};
use log::{debug, error};
use nom::character::complete::line_ending;
use nom::combinator::{map, opt};
use nom::error::{context, VerboseError};
use nom::sequence::{terminated, tuple};
use nom::IResult;
use std::fmt;
use std::fmt::Formatter;

const MAX_I16: i32 = 32768;
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
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = vec![];
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => match code {
                    _ => {
                        let b: u8 = (*code).into();
                        results.push(b);
                    }
                },
                _ => {
                    error!("Non-opcode found in opcode field");
                }
            }
        }

        for operand in vec![&self.operand_1, &self.operand_2, &self.operand_3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, symbols);
            }
        }

        // pad instructions that don't use full 32 bits
        while results.len() < 4 {
            results.push(0);
        }

        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                // using little-endian
                let mut wtr = vec![];
                wtr.write_i16::<LittleEndian>(*value as i16).unwrap();
                results.push(wtr[0]);
                results.push(wtr[1]);
            }
            Token::LabelUsage { name } => {
                if let Some(value) = symbols.symbol_value(name) {
                    let mut wtr = vec![];
                    wtr.write_u32::<LittleEndian>(value).unwrap();
                    results.push(wtr[0]);
                    results.push(wtr[1]);
                } else {
                    error!("No value found for {:?}", name);
                }
            }
            _ => {
                error!("Opcode found in operand field");
            }
        }
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_integer_needs_splitting(&self) -> bool {
        if let Some(ref op) = self.opcode {
            match op {
                Token::Op { code } => {
                    return match code {
                        Opcode::LOAD => {
                            if let Some(ref first_half) = self.operand_2 {
                                return match first_half {
                                    Token::IntegerOperand { ref value } => {
                                        if *value > MAX_I16 || *value < MIN_I16 {
                                            return true;
                                        }
                                        false
                                    }
                                    _ => false,
                                };
                            }
                            true
                        }
                        _ => false,
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn get_integer_value(&self) -> Option<i16> {
        if let Some(ref operand) = self.operand_2 {
            return match operand {
                Token::IntegerOperand { ref value } => Some(*value as i16),
                _ => None,
            };
        }
        None
    }

    pub fn get_register_number(&self) -> Option<u8> {
        match self.operand_1 {
            Some(ref reg_token) => match reg_token {
                Token::Register { ref reg_num } => Some(reg_num.clone()),
                _ => return None,
            },
            None => None,
        }
    }

    pub fn set_operand_two(&mut self, token: Token) {
        self.operand_2 = Some(token)
    }

    pub fn set_operand_three(&mut self, token: Token) {
        self.operand_3 = Some(token)
    }

    /// Checks if the AssemblyInstruction has any operands
    pub fn has_operands(&self) -> bool {
        self.operand_1.is_some() || self.operand_2.is_some() || self.operand_3.is_some()
    }

    pub fn get_directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(d) => match d {
                Token::Directive { name } => Some(name.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_string_constant(&self) -> Option<String> {
        match &self.operand_1 {
            Some(d) => match d {
                Token::LString { name } => Some(name.to_string()),
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
}

pub fn parse_instruction(input: &str) -> IResult<&str, AssemblerInstruction, VerboseError<&str>> {
    debug!("parse_instruction(\"{}\")", input);
    let mut parser = context(
        "Parsing a combined instruction",
        map(
            terminated(
                tuple((
                    opt(parse_separator),
                    opt(parse_comment),
                    opt(parse_separator),
                    opt(parse_label_declaration),
                    opt(parse_separator),
                    parse_opcode,
                    opt(parse_separator),
                    opt(parse_operand),
                    opt(parse_separator),
                    opt(parse_operand),
                    opt(parse_separator),
                    opt(parse_operand),
                    opt(parse_separator),
                    opt(parse_comment),
                )),
                opt(line_ending),
            ),
            |(
                _,
                _start_comment,
                _,
                label,
                _,
                opcode,
                _,
                operand_1,
                _,
                operand_2,
                _,
                operand_3,
                _,
                _end_comment,
            )| {
                AssemblerInstruction {
                    opcode: Some(opcode),
                    operand_1,
                    operand_2,
                    operand_3,
                    label,
                    directive: None,
                }
            },
        ),
    );

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("instruction parser error: {}", err);
    }

    return result;
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

#[cfg(test)]
mod tests {
    use crate::assembler::instruction_parsers::{parse_instruction, AssemblerInstruction};
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = parse_instruction("HLT");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    operand_1: None,
                    operand_2: None,
                    operand_3: None,
                    label: None,
                    directive: None,
                }
            ))
        );

        let result = parse_instruction("HLT\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    operand_1: None,
                    operand_2: None,
                    operand_3: None,
                    label: None,
                    directive: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_two() {
        println!("FORM 2");
        let result = parse_instruction("LOAD $0 #100");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand_1: Some(Token::Register { reg_num: 0 }),
                    operand_2: Some(Token::IntegerOperand { value: 100 }),
                    operand_3: None,
                    label: None,
                    directive: None,
                }
            ))
        );

        let result = parse_instruction("LOAD $0 #100\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand_1: Some(Token::Register { reg_num: 0 }),
                    operand_2: Some(Token::IntegerOperand { value: 100 }),
                    operand_3: None,
                    label: None,
                    directive: None,
                }
            ))
        );

        let result = parse_instruction("ADD $0 $1 $2");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    operand_1: Some(Token::Register { reg_num: 0 }),
                    operand_2: Some(Token::Register { reg_num: 1 }),
                    operand_3: Some(Token::Register { reg_num: 2 }),
                    label: None,
                    directive: None,
                }
            ))
        );

        let result = parse_instruction("ADD $0 $1 $2\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    operand_1: Some(Token::Register { reg_num: 0 }),
                    operand_2: Some(Token::Register { reg_num: 1 }),
                    operand_3: Some(Token::Register { reg_num: 2 }),
                    label: None,
                    directive: None,
                }
            ))
        );

        let result = parse_instruction("ADD $0 $1 $2 ; this is a comment\n");
        assert_eq!(
            result,
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::ADD }),
                    operand_1: Some(Token::Register { reg_num: 0 }),
                    operand_2: Some(Token::Register { reg_num: 1 }),
                    operand_3: Some(Token::Register { reg_num: 2 }),
                    label: None,
                    directive: None,
                }
            ))
        );
    }
}
