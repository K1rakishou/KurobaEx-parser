use crate::html_parser::node::Node;
use linked_hash_map::LinkedHashMap;
use std::fmt;

const CLASS_ATTR: &str = "class";

#[derive(Clone, PartialEq)]
pub struct Element {
  pub name: String,
  pub attributes: LinkedHashMap<String, String>,
  pub children: Vec<Node>,
  pub is_void_element: bool,
}

impl Element {
  pub fn has_class(&self, class_name: &String) -> bool {
    let class_attr_maybe = self.attributes.get(CLASS_ATTR);
    if class_attr_maybe.is_none() {
      return false;
    }

    return class_attr_maybe.unwrap().to_lowercase() == class_name.to_lowercase();
  }
}

impl fmt::Display for Element {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Element(name={}, attributes={}, children={}, is_void_element={})",
      self.name,
      self.attributes.len(),
      self.children.len(),
      self.is_void_element
    )
  }
}