#[derive(Debug, PartialEq)]
pub enum Token {
  AdditionOperator,
  SubtractionOperator,
  MultiplicationOperator,
  DivisionOperator,
  Integer { value: i64 },
  Float { value: f64 },
  Expression { left: Box<Token>, op: Box<Token>, right: Box<Token> },
  Program { statements: Vec<Token> },
}

// https://blog.subnetzero.io/post/building-language-vm-part-19/