use crate::rules::rule_handler::RuleHandler;
use crate::comment_parser::parser::Spannable;
use crate::html_parser::element::Element;

pub struct LineBreakRuleHandler {}

impl LineBreakRuleHandler {
  pub fn new() -> LineBreakRuleHandler {
    return LineBreakRuleHandler {};
  }
}

impl RuleHandler for LineBreakRuleHandler {
  fn handle(&self, element: &Element, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) -> bool {
    out_text_parts.push(String::from('\n'));
    return true;
  }
}