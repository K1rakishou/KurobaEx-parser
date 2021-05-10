use std::fmt;
use crate::Element;

#[derive(Clone, PartialEq)]
pub enum Node {
  Text(String),
  Element(Element)
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Node::Text(text) => {
        write!(f, "Text(text={})", text)
      }
      Node::Element(element) => {
        write!(f, "Element(element={})", element)
      }
    }
  }
}