use std::fmt;

#[derive(Debug, Clone)]
pub struct ParsingError {
  msg: String
}

impl fmt::Display for ParsingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}