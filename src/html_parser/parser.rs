use crate::html_parser::node::Node;
use std::str::Chars;
use std::io::Bytes;
use std::str;
use crate::html_parser::element::Element;
use std::collections::HashMap;

struct TextCursor<'a> {
  bytes: &'a [u8],
  offset: usize,
  length: usize
}

impl TextCursor<'_> {

  fn new(text: &str) -> TextCursor {
    return TextCursor {
      bytes: text.as_bytes(),
      offset: 0,
      length: text.len()
    }
  }

  fn advance(&mut self) {
    self.offset += 1;
  }

  fn advance_by(&mut self, count: usize) {
    self.offset += count;
  }

  fn skip_until(&mut self, ch: char) {
    while self.offset < self.length {
      let character = self.bytes[self.offset] as char;
      if character == ch {
        return;
      }

      self.advance();
    }
  }

  fn slice(&self, from: usize, len: usize) -> &str {
    return str::from_utf8(&self.bytes[from..(from + len)])
      .unwrap();
  }

  fn slice_until(&mut self, marker: &str) -> &str {
    let start = self.offset;

    while self.offset < self.length {
      let mut all_match = true;

      for (index, expected_char) in marker.char_indices() {
        let actual_char = self.bytes[self.offset + index] as char;

        if expected_char != actual_char {
          all_match = false;
          break
        }
      }

      if (all_match) {
        break;
      }

      self.advance();
    }

    return str::from_utf8(&self.bytes[start..self.offset])
      .unwrap();
  }

  fn current_offset(&self) -> usize {
    return self.offset;
  }

  fn current_char(&self) -> char {
    return self.bytes[self.offset] as char;
  }

  fn peek(&self, offset_relative: usize) -> char {
    return self.bytes[self.offset + offset_relative] as char;
  }

  fn reached_end(&self) -> bool {
    return self.offset >= self.length;
  }

}

pub struct HtmlParser {}

impl HtmlParser {

  pub fn new() -> HtmlParser {
    return HtmlParser {};
  }

  pub fn parse(&self, html: &str) -> Result<Vec<Node>, &str> {
    let mut out_nodes: Vec<Node> = Vec::with_capacity(16);
    let mut cursor = TextCursor::new(html);

    self.parse_internal(&mut cursor, &mut out_nodes);

    return Result::Ok(out_nodes);
  }

  fn parse_internal(&self, cursor: &mut TextCursor, out_nodes: &mut Vec<Node>) {
    let mut start: usize = 0;

    while (!cursor.reached_end()) {
      let current_char = cursor.current_char();
      if current_char == '<' && cursor.peek(1) == '/' {
        self.parse_raw_text(start, cursor, out_nodes);

        cursor.skip_until('>');
        cursor.advance();

        start = cursor.current_offset();
        continue;
      }

      if current_char == '<' {
        self.parse_raw_text(start, cursor, out_nodes);

        self.parse_tag(cursor, out_nodes);
        start = cursor.current_offset();
        continue;
      }

      cursor.advance();
    }

    self.parse_raw_text(start, cursor, out_nodes);
  }

  fn parse_tag(&self, cursor: &mut TextCursor, out_nodes: &mut Vec<Node>) {
    cursor.advance();

    let tag_full = cursor.slice_until(&">");
    let mut tag_parts = tag_full.split(&" ").collect::<Vec<&str>>();

    if (!tag_parts.is_empty()) {
      let mut tag_name: Option<String> = Option::None;
      let mut attributes: HashMap<String, Option<String>> = HashMap::new();

      for tag_part in tag_parts {
        if !tag_part.contains("=") {
          tag_name = Option::Some(String::from(tag_part));
          continue
        }

        let (attribute_split_vec) = tag_part.split("=").collect::<Vec<&str>>();
        let attr_name = attribute_split_vec[0];
        let attr_value = attribute_split_vec[1];

        attributes.insert(String::from(attr_name), Option::Some(String::from(attr_value)));

        println!("attr_name={}, attr_value={}", attr_name, attr_value);
      }

      if tag_name.is_some() {
        let element = Element {
          name: tag_name.unwrap(),
          attributes: attributes,
          children: Vec::new()
        };

        out_nodes.push(Node::Element(element));
      }
    }

    // Skip the ">"
    cursor.advance();
  }

  fn parse_raw_text(&self, start: usize, cursor: &mut TextCursor, out_nodes: &mut Vec<Node>) {
    if start == cursor.current_offset() {
      return;
    }

    let raw_text = cursor.slice(start, cursor.current_offset() - start);
    out_nodes.push(Node::Text(String::from(raw_text)));

    println!("raw_text={}", raw_text);
  }

}