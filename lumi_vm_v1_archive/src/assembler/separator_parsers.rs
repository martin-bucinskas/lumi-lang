use crate::assembler::Token;
use log::debug;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::error::{context, VerboseError};
use nom::IResult;

pub fn parse_separator(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_separator(\"{}\")", input);
    let mut parser = context("Parsing separator", map(space1, |_| Token::Separator));

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("separator parser error: {}", err);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::assembler::separator_parsers::parse_separator;
    use crate::assembler::Token;

    #[test]
    fn test_parse_separator() {
        let result = parse_separator("");
        assert_eq!(result.is_ok(), false);

        let result = parse_separator(" ");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(token, Token::Separator);

        let result = parse_separator("\t");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(token, Token::Separator);

        let result = parse_separator("\t ");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(token, Token::Separator);

        let result = parse_separator(" \t ");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(token, Token::Separator);

        let result = parse_separator("  ");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(token, Token::Separator);

        let result = parse_separator("\n");
        assert_eq!(result.is_ok(), false);

        let result = parse_separator(" \n");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(rest, "\n");
        assert_eq!(token, Token::Separator);
    }
}
