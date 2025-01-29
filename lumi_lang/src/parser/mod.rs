mod arithmetic;
mod tokens;
mod operand;

mod tests {
  use super::*;
  use tokens::Token;
  use crate::parser::operand::integer_operand;

  #[test]
  fn test_parse_integer() {
    let test_integers = vec!["0", "-1", "1"];
    for o in test_integers {
      let parsed_o = o.parse::<i64>().unwrap();
      let result = integer_operand(o);
      assert_eq!(result.is_ok(), true);
      let (_, token) = result.unwrap();
      assert_eq!(token, Token::Integer { value: parsed_o });
    }
  }
}