use crate::rules::rule_handler::RuleHandler;
use crate::{PostRaw, PostParserContext, Element, TextPart, Spannable};

const TAG: &str = "AbbrHandler";

pub struct AbbrHandler {}

impl AbbrHandler {
  pub fn new() -> AbbrHandler {
    return AbbrHandler {};
  }
}

impl RuleHandler for AbbrHandler {

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