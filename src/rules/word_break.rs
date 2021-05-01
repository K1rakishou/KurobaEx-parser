use crate::rules::rule_handler::RuleHandler;
use crate::comment_parser::comment_parser::Spannable;
use crate::html_parser::element::Element;
use crate::post_parser::post_parser::PostParserContext;
use crate::PostRaw;

pub struct WordBreakRuleHandler {}

impl WordBreakRuleHandler {
  pub fn new() -> WordBreakRuleHandler {
    return WordBreakRuleHandler {};
  }
}

impl RuleHandler for WordBreakRuleHandler {

  fn handle(&self, _: &PostRaw, _: &PostParserContext, _: &Element, _: &mut Vec<String>, _: &mut Vec<Spannable>) -> bool {
    return true;
  }

  fn post_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: usize,
    _: &mut Vec<String>,
    _: usize,
    _: &mut Vec<Spannable>
  ) {
    // no-op
  }

}