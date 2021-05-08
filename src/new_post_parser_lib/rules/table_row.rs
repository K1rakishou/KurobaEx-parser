use crate::rules::rule_handler::RuleHandler;
use crate::{PostRaw, PostParserContext, Element, TextPart, Spannable};

pub struct TableRowHandler {}

impl TableRowHandler {
  pub fn new() -> TableRowHandler {
    return TableRowHandler {};
  }
}

impl RuleHandler for TableRowHandler {

  fn pre_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: &mut Vec<TextPart>,
    _: &mut Vec<Spannable>
  ) -> bool {
    return false;
  }

  fn post_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: usize,
    out_text_parts: &mut Vec<TextPart>,
    _: usize,
    _: &mut Vec<Spannable>
  ) {
    out_text_parts.push(TextPart::new(String::from('\n')));
  }

}