use crate::assembler::Token;
use log::debug;
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, recognize};
use nom::error::{context, VerboseError};
use nom::sequence::preceded;
use nom::IResult;

pub fn parse_register(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_register(\"{}\")", input);
    let mut parser = context(
        "Parsing a register value",
        map_res(
            preceded(char('$'), recognize(digit1)),
            |reg_num_str: &str| {
                reg_num_str
                    .parse::<u8>()
                    .map(|reg_num| Token::Register { reg_num })
                    .map_err(|_| "Unable to parse register number")
            },
        ),
    );

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("register parser error: {}", err);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::assembler::register_parsers::parse_register;
    use crate::assembler::Token;

    #[test]
    fn test_register_parser() {
        let result = parse_register("$1");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Register { reg_num: 1 });
        assert_eq!(rest, "");

        let result = parse_register("0");
        assert_eq!(result.is_ok(), false);

        let result = parse_register("$a");
        assert_eq!(result.is_ok(), false);
    }
}
