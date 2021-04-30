use crate::html_parser::node::Node;
use std::str::Chars;
use std::io::Bytes;
use std::str;
use crate::html_parser::element::Element;
use std::collections::{HashSet};
use linked_hash_map::LinkedHashMap;
use serde::de::Unexpected::Str;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::borrow::BorrowMut;

lazy_static! {
  static ref VOID_ELEMENTS: HashSet<&'static str> = {
    let mut set = HashSet::new();

    set.insert("area");
    set.insert("base");
    set.insert("br");
    set.insert("wbr");
    set.insert("col");
    set.insert("hr");
    set.insert("img");
    set.insert("input");
    set.insert("link");
    set.insert("meta");
    set.insert("param");

    return set;
  };
}

pub struct HtmlParser {}

struct HtmlParserContext {
  parsing_tag: bool,
}

impl HtmlParserContext {
  pub fn new(parsing_tag: bool) -> HtmlParserContext {
    return HtmlParserContext { parsing_tag };
  }
}

impl HtmlParser {
  pub fn new() -> HtmlParser {
    return HtmlParser {};
  }

  pub fn parse(&self, html: &str) -> Result<Vec<Node>, &str> {
    let (result_nodes, result_offset) = self.parse_internal(
      html.as_bytes(),
      0,
    );

    return Result::Ok(result_nodes);
  }

  fn parse_internal(&self, html: &[u8], start: usize) -> (Vec<Node>, usize) {
    let mut out_nodes: Vec<Node> = Vec::new();
    let mut local_offset = start;
    let mut current_buffer = Vec::new();

    while (local_offset < html.len()) {
      let curr_char = html[local_offset as usize] as char;

      if (curr_char == '<') {
        if (current_buffer.len() > 0) {
          out_nodes.push(Node::Text(String::from_iter(&current_buffer)));
          current_buffer.clear();
        }

        local_offset += 1;

        let next_char = html[local_offset as usize] as char;
        if (next_char == '/') {
          let offset = self.skip_tag_end(html, local_offset);
          local_offset = offset;

          return (out_nodes, local_offset);
        }

        let (element, offset) = self.parse_tag(html, local_offset);
        out_nodes.push(Node::Element(element));
        local_offset = offset;

        continue;
      }

      current_buffer.push(curr_char);
      local_offset += 1;
    }

    if (current_buffer.len() > 0) {
      out_nodes.push(Node::Text(String::from_iter(&current_buffer)));
      current_buffer.clear();
    }

    return (out_nodes, local_offset);
  }

  fn parse_tag(&self, html: &[u8], start: usize) -> (Element, usize) {
    let mut local_offset = start;
    let mut tag_raw: Vec<char> = Vec::with_capacity(32);

    while (local_offset < html.len()) {
      let ch = html[local_offset as usize] as char;
      if (ch == '>') {
        break;
      }

      tag_raw.push(ch);
      local_offset += 1;
    }

    // Skip the ">"
    local_offset += 1;

    let element = self.create_tag(&String::from_iter(tag_raw));
    if (element.is_void_element) {
      return (element, local_offset);
    }

    let (child_nodes, new_offset) = self.parse_internal(
      html,
      local_offset,
    );

    let updated_element = Element {
      name: element.name,
      attributes: element.attributes,
      children: child_nodes,
      is_void_element: false,
    };

    return (updated_element, new_offset);
  }

  fn skip_tag_end(&self, html: &[u8], start: usize) -> usize {
    let mut local_offset = start;

    while (local_offset < html.len()) {
      let ch = html[local_offset as usize] as char;
      if (ch == '>') {
        return local_offset + 1;
      }

      local_offset += 1;
    }

    panic!("Failed to find tag end");
  }

  fn create_tag(&self, tag_raw: &String) -> Element {
    let mut tag_parts = tag_raw.split(&" ").collect::<Vec<&str>>();

    if (tag_parts.is_empty()) {
      panic!("tag_parts is empty! tag_raw={}", tag_raw);
    }

    let mut tag_name_maybe: Option<String> = Option::None;
    let mut attributes: LinkedHashMap<String, Option<String>> = LinkedHashMap::new();

    for tag_part in tag_parts {
      if !tag_part.contains("=") {
        tag_name_maybe = Option::Some(String::from(tag_part));
        continue;
      }

      let (attribute_split_vec) = tag_part.split("=").collect::<Vec<&str>>();
      let attr_name = attribute_split_vec[0];
      let attr_value = attribute_split_vec[1];

      attributes.insert(String::from(attr_name), Option::Some(String::from(attr_value)));
    }

    if tag_name_maybe.is_none() {
      panic!("Tag has no name!")
    }

    let tag_name = tag_name_maybe.unwrap();
    let is_void_element = VOID_ELEMENTS.contains(&tag_name.as_str());

    return Element {
      name: tag_name,
      attributes: attributes,
      children: Vec::new(),
      is_void_element: is_void_element
    };
  }

  pub fn debug_print_nodes(&self, nodes: &Vec<Node>) {
    self.debug_print_nodes_internal(
      nodes,
      0,
      &mut |node_string: String| { println!("{}", node_string) }
    );
  }

  fn debug_print_nodes_internal(&self, nodes: &Vec<Node>, depth: usize, iterator: &mut dyn FnMut(String)) {
    for node in nodes {
      match node {
        Node::Text(text) => {
          iterator(format!("{}{}", self.format_depth(depth), text));
        }
        Node::Element(element) => {
          iterator(
            format!(
              "{}<{}{}>",
              self.format_depth(depth),
              &element.name,
              self.debug_format_attributes(&element.attributes)
            )
          );

          self.debug_print_nodes_internal(&element.children, depth + 1, iterator);
        }
      }
    }
  }

  pub fn debug_concat_into_string(&self, nodes: &Vec<Node>) -> String {
    let mut result_string = String::new();

    self.debug_concat_into_string_internal(
      nodes,
      &mut |node_string: String| { result_string.push_str(node_string.as_str()) }
    );

    return result_string;
  }

  pub fn debug_concat_into_string_internal(&self, nodes: &Vec<Node>, iterator: &mut dyn FnMut(String)) {
    for node in nodes {
      match node {
        Node::Text(text) => {
          iterator(format!("{}", text));
        }
        Node::Element(element) => {
          iterator(format!("<{}{}>", &element.name, self.debug_format_attributes(&element.attributes)));
          self.debug_concat_into_string_internal(&element.children, iterator);
        }
      }
    }
  }

  fn format_depth(&self, current_depth: usize) -> String {
    let mut result_string = String::new();

    for i in 0..current_depth {
      result_string.push_str(" ");
    }

    return result_string;
  }

  fn debug_format_attributes(&self, attributes: &LinkedHashMap<String, Option<String>>) -> String {
    let mut result_string = String::new();

    if (attributes.is_empty()) {
      return result_string;
    }

    let mut index = 0;

    for (attr_key, attr_value_maybe) in attributes {
      let attr_value = match attr_value_maybe {
        None => "null",
        Some(value) => value
      };

      result_string.push_str(format!(", {}={}", attr_key, attr_value).as_str());
      index += 1;
    }

    return result_string;
  }
}