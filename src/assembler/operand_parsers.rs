use crate::assembler::label_parsers::parse_label_usage;
use crate::assembler::register_parsers::parse_register;
use crate::assembler::Token;
use log::{debug, error};
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{map, not, opt, peek, recognize};
use nom::error::{context, VerboseError};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn parse_operand(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_operand(\"{}\")", input);
    let mut parser = context(
        "Parsing an operand",
        alt((
            parse_integer_operand,
            parse_float_operand,
            parse_label_usage,
            parse_register,
            parse_lstring,
        )),
    );

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("operand parser error: {}", err);
    }

    return result;
}

fn parse_integer_operand(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_integer_operand(\"{}\")", input);
    context(
        "Parsing an integer operand",
        map(
            preceded(
                char('#'),
                recognize(tuple((opt(char('-')), digit1, peek(not(char('.')))))),
            ),
            |num_str: &str| Token::IntegerOperand {
                value: num_str.parse::<i32>().unwrap(),
            },
        ),
    )(input)
}

fn parse_float_operand(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_float_operand(\"{}\")", input);
    context(
        "Parsing a float operand",
        map(
            preceded(
                char('#'),
                recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
            ),
            |num_str: &str| Token::FloatOperand {
                value: num_str.parse::<f64>().unwrap(),
            },
        ),
    )(input)
}

fn parse_lstring(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_irstring(\"{}\")", input);
    context(
        "Parsing an IrString operand",
        map(
            preceded(char('\''), terminated(take_until("'"), char('\''))),
            |content: &str| Token::LString {
                name: content.to_string(),
            },
        ),
    )(input)
}

mod tests {
    use crate::assembler::operand_parsers::{
        parse_float_operand, parse_integer_operand, parse_lstring, parse_operand,
    };
    use crate::assembler::Token;

    #[test]
    fn test_integer_operand() {
        let result = parse_integer_operand("#1");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::IntegerOperand { value: 1 });
        assert_eq!(rest, "");
    }

    #[test]
    fn test_negative_integer_operand() {
        let result = parse_integer_operand("#-1");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::IntegerOperand { value: -1 });
        assert_eq!(rest, "");
    }

    #[test]
    fn test_no_match_integer_operand() {
        let result = parse_integer_operand("0");
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().to_string(),
            "Parsing Error: VerboseError { errors: [(\"0\", Char('#')), (\"0\", Context(\"Parsing an integer operand\"))] }"
        );

        let result = parse_integer_operand("a");
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().to_string(),
            "Parsing Error: VerboseError { errors: [(\"a\", Char('#')), (\"a\", Context(\"Parsing an integer operand\"))] }"
        );
    }

    #[test]
    fn test_float_operand() {
        let result = parse_float_operand("#1.123");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::FloatOperand { value: 1.123 });
        assert_eq!(rest, "");
    }

    #[test]
    fn test_negative_float_operand() {
        let result = parse_float_operand("#-1.123");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::FloatOperand { value: -1.123 });
        assert_eq!(rest, "");
    }

    #[test]
    fn test_no_match_float_operand() {
        let result = parse_float_operand("0");
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().to_string(),
            "Parsing Error: VerboseError { errors: [(\"0\", Char('#')), (\"0\", Context(\"Parsing a float operand\"))] }"
        );

        let result = parse_float_operand("a");
        assert_eq!(result.is_ok(), false);
        assert_eq!(
            result.err().unwrap().to_string(),
            "Parsing Error: VerboseError { errors: [(\"a\", Char('#')), (\"a\", Context(\"Parsing a float operand\"))] }"
        );
    }

    #[test]
    fn test_parse_string_operand() {
        let result = parse_lstring("'this is a test'");
        assert_eq!(result.is_ok(), true);

        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::LString {
                name: "this is a test".to_string()
            }
        );
    }

    #[test]
    fn test_combined_integer() {
        let result = parse_operand("#0");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(token, Token::IntegerOperand { value: 0 });
    }

    #[test]
    fn test_combined_integer_negative() {
        let result = parse_operand("#-1");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(token, Token::IntegerOperand { value: -1 });
    }

    #[test]
    fn test_combined_float() {
        let result = parse_operand("#0.123");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(token, Token::FloatOperand { value: 0.123 });
    }

    #[test]
    fn test_combined_float_negative() {
        let result = parse_operand("#-0.123");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(token, Token::FloatOperand { value: -0.123 });
    }

    #[test]
    fn test_combined_label_usage() {
        let result = parse_operand("@label");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(
            token,
            Token::LabelUsage {
                name: "label".to_string()
            }
        );
    }

    #[test]
    fn test_combined_register() {
        let result = parse_operand("$1");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(token, Token::Register { reg_num: 1 });
    }

    #[test]
    fn test_combined_lstring() {
        let result = parse_operand("'hello world'");
        assert_eq!(result.is_ok(), true);

        let (rest, token) = result.unwrap();
        assert_eq!("", rest);
        assert_eq!(
            token,
            Token::LString {
                name: "hello world".to_string()
            }
        );
    }
}
