use crate::html_parser::node::Node;
use linked_hash_map::LinkedHashMap;

const CLASS_ATTR: &str = "class";

#[derive(Debug, Clone, PartialEq)]
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