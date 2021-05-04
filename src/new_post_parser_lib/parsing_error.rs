use std::fmt;

#[derive(Debug, Clone)]
pub struct ParsingError {
  msg: String
}

impl ParsingError {
  pub fn new(msg: &str) -> ParsingError {
    return ParsingError { msg: String::from(msg) }
  }
}

impl fmt::Display for ParsingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}