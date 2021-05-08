use crate::html_parser::node::Node;
use std::fmt;
use crate::Element;

const CLASS_ATTR: &str = "class";

impl Element {
  pub fn has_class(&self, class_name: &str) -> bool {
    let class_attr_maybe = self.attributes.get(CLASS_ATTR);
    if class_attr_maybe.is_none() {
      return false;
    }

    return class_attr_maybe.unwrap().to_lowercase() == class_name.to_lowercase();
  }

  pub fn get_attr_value(&self, attr_name: &str) -> Option<&String> {
    return self.attributes.get(attr_name);
  }

  pub fn collect_text(&self) -> String {
    let mut output = String::with_capacity(16);

    for child in self.children.iter() {
      self.collect_text_internal(child, &mut output);
    }

    return output;
  }

  fn collect_text_internal(&self, node: &Node, output: &mut String) {
    match node {
      Node::Text(text) => output.push_str(text),
      Node::Element(element) => {
        for child in element.children.iter() {
          self.collect_text_internal(&child, output);
        }
      }
    }
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