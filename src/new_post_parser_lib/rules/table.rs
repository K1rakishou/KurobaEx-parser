use crate::rules::rule_handler::{RuleHandler, RuleHandlerPostHandleMeta};
use crate::{PostRaw, PostParserContext, Element, TextPart, Spannable, SpannableData};

pub struct TableHandler {}

impl TableHandler {
  pub fn new() -> TableHandler {
    return TableHandler {};
  }
}

impl RuleHandler for TableHandler {

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
    out_spannables: &mut Vec<Spannable>
  ) {
    let start = (self as &dyn RuleHandler).get_out_text_parts_diff_len(
      prev_out_text_parts_index,
      &out_text_parts
    ) as usize;

    let len = (self as &dyn RuleHandler).get_out_text_parts_new_len(
      prev_out_text_parts_index,
      &out_text_parts
    ) as usize;

    let spannable = Spannable {
      start,
      len,
      spannable_data: SpannableData::Monospace
    };

    if spannable.is_valid() {
      out_spannables.push(spannable);
    }
  }

}