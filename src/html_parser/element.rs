use crate::html_parser::node::Node;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
  pub name: String,
  pub attributes: LinkedHashMap<String, Option<String>>,
  pub children: Vec<Node>,
  pub is_void_element: bool
}