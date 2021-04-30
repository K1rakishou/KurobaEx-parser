use crate::rules::rule_handler::RuleHandler;
use crate::comment_parser::parser::Spannable;
use crate::html_parser::element::Element;

pub struct WordBreakRuleHandler {}

impl WordBreakRuleHandler {
  pub fn new() -> WordBreakRuleHandler {
    return WordBreakRuleHandler {};
  }
}

impl RuleHandler for WordBreakRuleHandler {
  fn handle(&self, _: &Element, _: &mut Vec<String>, _: &mut Vec<Spannable>) -> bool {
    return true;
  }
}