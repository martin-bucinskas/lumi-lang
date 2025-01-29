use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::error::{context, VerboseError};
use nom::IResult;
use nom::sequence::{delimited, preceded};
use crate::parser::tokens::Token;

pub fn arithmetic_operator(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  let mut parser = context(
    "Parsing an opcode",
    delimited(
      multispace0,
      alt((
        addition_operator,
        subtraction_operator,
        multiplication_operator,
        division_operator,
      )),
      multispace0,
    ),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    // debug!("opcode parser error: {}", err);
  }

  return result;
}

pub fn addition_operator(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  let mut parser = context(
    "Parsing an opcode",
    preceded(
      multispace0,
      map(
        tag("+"),
        |_| Token::AdditionOperator,
      ),
    ),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    // debug!("opcode parser error: {}", err);
  }

  return result;
}

pub fn subtraction_operator(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  let mut parser = context(
    "Parsing an opcode",
    preceded(
      multispace0,
      map(
        tag("-"),
        |_| Token::SubtractionOperator,
      ),
    ),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    // debug!("opcode parser error: {}", err);
  }

  return result;
}

pub fn multiplication_operator(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  let mut parser = context(
    "Parsing an opcode",
    preceded(
      multispace0,
      map(
        tag("*"),
        |_| Token::MultiplicationOperator,
      ),
    ),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    // debug!("opcode parser error: {}", err);
  }

  return result;
}

pub fn division_operator(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  let mut parser = context(
    "Parsing an opcode",
    preceded(
      multispace0,
      map(
        tag("/"),
        |_| Token::DivisionOperator,
      ),
    ),
  );

  let result = parser(input);

  if result.is_err() {
    let err = result.as_ref().err().unwrap();
    // debug!("opcode parser error: {}", err);
  }

  return result;
}