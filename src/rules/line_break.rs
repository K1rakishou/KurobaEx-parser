use crate::rules::rule_handler::RuleHandler;
use html_parser::Element;
use crate::comment_parser::parser::Spannable;

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