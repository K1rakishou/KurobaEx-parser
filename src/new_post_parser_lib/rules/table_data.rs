use crate::rules::rule_handler::{RuleHandler, RuleHandlerPostHandleMeta};
use crate::{PostRaw, PostParserContext, Element, TextPart, Spannable};

pub struct TableDataHandler {}

impl TableDataHandler {
  pub fn new() -> TableDataHandler {
    return TableDataHandler {};
  }
}

impl RuleHandler for TableDataHandler {

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
    prev_out_text_parts_index: usize,
    out_text_parts: &mut Vec<TextPart>,
    _: usize,
    _: &mut Vec<Spannable>
  ) {
    let text = (self as &dyn RuleHandler).get_out_text_parts_diff_text(
      prev_out_text_parts_index,
      &out_text_parts
    );

    let mut only_contains_whitespaces = true;

    for char in text.chars() {
      if !char.is_whitespace() {
        only_contains_whitespaces = false;
        break;
      }
    }

    if only_contains_whitespaces {
      return;
    }

    out_text_parts.push(TextPart::new(String::from(' ')));
  }

}