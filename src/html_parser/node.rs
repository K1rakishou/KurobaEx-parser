use crate::html_parser::element::Element;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
  Text(String),
  Element(Element)
}