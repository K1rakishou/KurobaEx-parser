use crate::rules::rule_handler::RuleHandler;
use crate::{PostRaw, PostParserContext, Element, Spannable, SpannableData};
use crate::util::helpers::SumBy;

const TAG: &str = "SpoilerHandler";

pub struct SpoilerHandler {}

impl SpoilerHandler {
  pub fn new() -> SpoilerHandler {
    return SpoilerHandler {};
  }
}

impl RuleHandler for SpoilerHandler {

  fn pre_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: &mut Vec<String>,
    _: &mut Vec<Spannable>
  ) -> bool {
    // We want to process <s> tag after it's children are processed since we need to know their
    // total text size
    return false;
  }

  fn post_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    prev_out_text_parts_index: usize,
    out_text_parts: &mut Vec<String>,
    _: usize,
    out_spannables: &mut Vec<Spannable>
  ) {
    if prev_out_text_parts_index == out_text_parts.len() {
      // Nothing was added since handle() call. This probably means that the current tag has an empty
      // body.
      eprintln!(
        "{} prev_out_text_parts_index ({}) == out_text_parts.len() ({})",
        TAG,
        prev_out_text_parts_index,
        out_text_parts.len()
      );

      return;
    }

    let start = out_text_parts[0..prev_out_text_parts_index]
      .iter()
      .sum_by(&|string| string.len() as i32);

    let len = out_text_parts[prev_out_text_parts_index..]
      .iter()
      .sum_by(&|string| string.len() as i32);

    let spannable = Spannable {
      start: start as usize,
      len: len as usize,
      spannable_data: SpannableData::Spoiler
    };

    if spannable.is_valid() {
      out_spannables.push(spannable);
    }
  }

}