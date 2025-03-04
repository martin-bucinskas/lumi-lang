use log::{debug};
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser, Debug)]
#[grammar = "grammar/lumi_asm_v2.pest"]
pub struct LumiAsmParser;

impl LumiAsmParser {
  pub fn print_pair(pair: Pair<Rule>, indent: usize) {
    let indent_str = " ".repeat(indent);
    debug!("{}Rule:  {:?}", indent_str, pair.as_rule());
    debug!("{}Span:  {:?}", indent_str, pair.as_span().as_str());
    for inner in pair.into_inner() {
      LumiAsmParser::print_pair(inner, indent + 1);
    }
  } 
}

#[cfg(test)]
mod tests {
  use super::*;
  use pest::Parser;
  use std::sync::Once;
  use log::LevelFilter;

  static INIT: Once = Once::new();

  fn init_logger() {
    INIT.call_once(|| {
      env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .is_test(true)
        .try_init()
        .ok();
    });
  }

  #[test]
  fn parse_comment() {
    init_logger();
    let input = "; This is a comment\n";
    let result = LumiAsmParser::parse(Rule::COMMENT, input);

    assert!(
      result.is_ok(),
      "Expected comment to parse successfully, but got: {:?}",
      result.err()
    );

    let pairs = result.unwrap();
    for pair in pairs {
      LumiAsmParser::print_pair(pair, 0);
    }
  }

  #[test]
  fn parse_directive_line() {
    init_logger();
    let input = ".integer 5\n";
    let result = LumiAsmParser::parse(Rule::program, input);

    assert!(
      result.is_ok(),
      "Expected program to parse, but got: {:?}",
      result.err()
    );

    let pairs = result.unwrap();
    for pair in pairs {
      LumiAsmParser::print_pair(pair, 0);
    }
  }
}
