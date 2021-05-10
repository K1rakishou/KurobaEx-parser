use crate::rules::rule_handler::RuleHandler;
use crate::{PostRaw, PostParserContext, Element, TextPart, Spannable, SpannableData};
use crate::util::helpers::SumBy;

const TAG: &str = "PreHandler";

pub struct PreHandler {}

impl PreHandler {
  pub fn new() -> PreHandler {
    return PreHandler {};
  }
}

impl RuleHandler for PreHandler {

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
    if prev_out_text_parts_index == out_text_parts.len() {
      // Nothing was added since handle() call. This probably means that the current tag has an empty
      // body.
      return;
    }

    let start = out_text_parts[0..prev_out_text_parts_index]
      .iter()
      .sum_by(&|string| string.characters_count as i32);

    let len = out_text_parts[prev_out_text_parts_index..]
      .iter()
      .sum_by(&|string| string.characters_count as i32);

    let spannable = Spannable {
      start: start as usize,
      len: len as usize,
      spannable_data: SpannableData::Monospace
    };

    if spannable.is_valid() {
      out_spannables.push(spannable);
    }
  }

}