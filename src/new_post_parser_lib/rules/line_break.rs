use crate::rules::rule_handler::RuleHandler;
use crate::{PostRaw, PostParserContext, Element, Spannable};

pub struct LineBreakRuleHandler {}

impl LineBreakRuleHandler {
  pub fn new() -> LineBreakRuleHandler {
    return LineBreakRuleHandler {};
  }
}

impl RuleHandler for LineBreakRuleHandler {

  fn pre_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    out_text_parts: &mut Vec<String>,
    _: &mut Vec<Spannable>
  ) -> bool {
    out_text_parts.push(String::from('\n'));
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