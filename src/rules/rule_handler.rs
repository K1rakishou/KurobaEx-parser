use html_parser::Element;
use crate::comment_parser::parser::Spannable;

pub trait RuleHandler {
  fn handle(&self, element: &Element, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) -> bool;
}