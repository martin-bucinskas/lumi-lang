use log::debug;
use nom::character::complete::{alphanumeric1, char, multispace0};
use nom::combinator::map;
use nom::error::{context, VerboseError};
use nom::IResult;
use nom::sequence::{pair, terminated};
use crate::assembler::Token;

pub fn parse_label_declaration(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_label_declaration(\"{}\")", input);
    let mut parser = context(
        "Parsing a label declaration",
        map(
            pair(alphanumeric1, terminated(char(':'), multispace0)),
            |(name, _): (&str, _)| Token::LabelDeclaration {
                name: name.to_string(),
            },
        ),
    );

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("label parser error: {}", err);
    }

    result
}

pub fn parse_label_usage(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_label_usage(\"{}\")", input);
    context(
        "Parsing a label usage",
        map(
            pair(char('@'), alphanumeric1),
            |(_, name): (char, &str)| Token::LabelUsage {
                name: name.to_string(),
            },
        ),
    )(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_label_declaration() {
    let result = parse_label_declaration("label:");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_label_usage() {
    let result = parse_label_usage("@label");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_label_usage_terminated() {
    let result = parse_label_usage("@label\n");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_parse_label_declaration_2() {
    let result = parse_label_declaration("load");
    assert_eq!(result.is_ok(), false);
  }
}
