use log::debug;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::combinator::{map, rest};
use nom::error::{context, VerboseError};
use nom::sequence::preceded;
use nom::IResult;
use crate::assembler::Token;

pub fn parse_comment(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  debug!("parse_comment(\"{}\")", input);
  let mut parser = context(
    "Parsing a comment",
    map(preceded(char(';'), alt((take_until("\n"), rest))), |_| {
      Token::Comment
    }),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    debug!("comment parser error: {}", err);
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn comment_parsing() {
    let result = parse_comment(";this is a comment\n");
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn comment_parsing_with_space() {
    let result = parse_comment(";this is a comment");
    assert_eq!(result.is_ok(), true);
  }
}
