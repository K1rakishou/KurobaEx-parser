use crate::html_parser::node::Node;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
  pub name: String,
  pub attributes: HashMap<String, Option<String>>,
  pub children: Vec<Node>,
}