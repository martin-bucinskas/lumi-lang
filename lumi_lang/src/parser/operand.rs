use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, i64, multispace0};
use nom::combinator::{map_res, opt};
use nom::error::{context, VerboseError, VerboseErrorKind};
use nom::IResult;
use nom::sequence::{delimited, pair};
use crate::parser::arithmetic::{addition_operator, division_operator, multiplication_operator, subtraction_operator};
use crate::parser::tokens::Token;

pub fn integer_operand(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
  let (remaining_input, (sign, num)) = delimited(
    multispace0,
    pair(
      opt(tag("-")),
      digit1,
    ),
    multispace0,
  )(input)?;

  // Build numerical String
  let mut num_string = String::from("");
  if let Some(_) = sign {
    num_string.push_str("-");
  }
  num_string.push_str(num);

  // Parse numerical String into an integer
  match i64::from_str(&num_string) {
    Ok(value) => Ok((remaining_input, Token::Integer { value })),
    Err(_) => {
      Err(VerboseError {
        errors: vec![(remaining_input, VerboseErrorKind::Context("Parse Error"))],
      })
    }
  }
}