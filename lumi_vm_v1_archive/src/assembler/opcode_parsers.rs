use crate::assembler::Token;
use crate::instruction::Opcode;
use log::debug;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::error::{context, VerboseError};
use nom::IResult;

pub fn parse_opcode(input: &str) -> IResult<&str, Token, VerboseError<&str>> {
    debug!("parse_opcode(\"{}\")", input);
    let mut parser = context(
        "Parsing an opcode",
        map(alpha1, |opcode: &str| Token::Op {
            code: Opcode::from(opcode),
        }),
    );

    let result = parser(input);

    if result.is_err() {
        let err = result.as_ref().err().unwrap();
        debug!("opcode parser error: {}", err);
    }

    return result;
}

mod tests {
    use crate::assembler::opcode_parsers::parse_opcode;
    use crate::assembler::Token;
    use crate::instruction::Opcode;

    #[test]
    fn test_opcode_load() {
        let result = parse_opcode("load");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = parse_opcode("aold");
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
        assert_eq!(rest, "");
    }
}
