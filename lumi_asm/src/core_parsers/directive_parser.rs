use log::debug;
use nom::character::complete::{alpha1, char, line_ending};
use nom::combinator::{map, opt};
use nom::error::{context, VerboseError};
use nom::IResult;
use nom::sequence::{preceded, terminated, tuple};
use crate::assembler::Token;
use crate::core_parsers::label_parser::parse_label_declaration;
use crate::core_parsers::operand_parser::parse_operand;
use crate::core_parsers::separator_parser::parse_separator;
use crate::parser_combinators::instruction_parser::AssemblerInstruction; // todo: move this out to local


fn parse_directive_declaration(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  debug!("parse_directive_declaration(\"{}\")", input);
  context(
    "Parsing a directive declaration",
    map(preceded(char('.'), alpha1), |name: &str| Token::Directive {
      name: name.to_string(),
    }),
  )(input)
}

pub fn parse_directive(input: &str) -> IResult<&str, AssemblerInstruction, VerboseError<&str>> {
    debug!("parse_directive_combined(\"{}\")", input);
    let mut parser = context(
        "Parsing a combined directive",
        map(
            terminated(
                tuple((
                    opt(parse_separator),
                    opt(parse_label_declaration),
                    opt(parse_separator),
                    parse_directive_declaration,
                    opt(parse_separator),
                    opt(parse_operand),
                    opt(parse_separator),
                    opt(parse_operand),
                    opt(parse_separator),
                    opt(parse_operand),
                )),
                opt(line_ending),
            ),
            |(
                _,
                label_declaration,
                _,
                directive_name,
                _,
                operand_1,
                _,
                operand_2,
                _,
                operand_3,
            )| {
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(directive_name),
                    label: label_declaration,
                    operand_1,
                    operand_2,
                    operand_3,
                }
            },
        ),
    );

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("directive parser error: {}", err);
    }

    result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_directive() {
    let result = parse_directive(".data");
    assert_eq!(result.is_ok(), true);
    let (_, instruction) = result.unwrap();
    assert_eq!(
      instruction.directive,
      Some(Token::Directive {
        name: "data".to_string()
      })
    );
  }

  #[test]
  fn test_string_directive() {
    let result = parse_directive("test: .asciiz 'hello'");
    assert_eq!(result.is_ok(), true);

    let (_, instruction) = result.unwrap();

    let expected_instruction = AssemblerInstruction {
      opcode: None,
      label: Some(Token::LabelDeclaration {
        name: "test".to_string(),
      }),
      directive: Some(Token::Directive {
        name: "asciiz".to_string(),
      }),
      operand_1: Some(Token::LString {
        value: "hello".to_string(),
      }),
      operand_2: None,
      operand_3: None,
    };

    assert_eq!(instruction, expected_instruction);
  }
}