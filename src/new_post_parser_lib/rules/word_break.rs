use crate::rules::rule_handler::RuleHandler;
use crate::{PostRaw, PostParserContext, Element, Spannable, TextPart};

pub struct WordBreakRuleHandler {}

impl WordBreakRuleHandler {
  pub fn new() -> WordBreakRuleHandler {
    return WordBreakRuleHandler {};
  }
}

impl RuleHandler for WordBreakRuleHandler {

  fn pre_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: &mut Vec<TextPart>,
    _: &mut Vec<Spannable>
  ) -> bool {
    return true;
  }

  fn post_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: usize,
    _: &mut Vec<TextPart>,
    _: usize,
    _: &mut Vec<Spannable>
  ) {
    // no-op
  }

}