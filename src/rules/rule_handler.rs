use crate::comment_parser::parser::Spannable;
use crate::html_parser::element::Element;

pub trait RuleHandler {
  fn handle(&self, element: &Element, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) -> bool;
}