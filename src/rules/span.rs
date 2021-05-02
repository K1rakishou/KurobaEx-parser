use crate::rules::rule_handler::{RuleHandler, RuleHandlerPostHandleMeta};
use crate::comment_parser::comment_parser::{Spannable, SpannableData, PostLink};
use crate::html_parser::element::Element;
use crate::post_parser::post_parser::PostParserContext;
use crate::PostRaw;
use crate::util::helpers::SumBy;

pub struct SpanHandler {}

impl SpanHandler {
  pub fn new() -> SpanHandler {
    return SpanHandler {};
  }
}

impl RuleHandler for SpanHandler {

  fn pre_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: &mut Vec<String>,
    _: &mut Vec<Spannable>
  ) -> bool {
    // We want to process <span> tag after it's children are processed since we need to know their
    // total text size
    return false;
  }

  fn post_handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    prev_out_text_parts_index: usize,
    out_text_parts: &mut Vec<String>,
    prev_out_spannables_index: usize,
    out_spannables: &mut Vec<Spannable>
  ) {
    if element.children.is_empty() {
      return;
    }

    if prev_out_text_parts_index < 0 || prev_out_text_parts_index == out_text_parts.len() {
      // Nothing was added since handle() call. This probably means that the current tag has an empty
      // body.
      return;
    }

    if element.has_class("quote") {
      // greentext
      let start = (self as &dyn RuleHandler).get_out_text_parts_diff_len(prev_out_text_parts_index, &out_text_parts);
      let len = (self as &dyn RuleHandler).get_out_text_parts_new_len(prev_out_text_parts_index, &out_text_parts) as usize;

      let spannable = Spannable {
        start,
        len,
        spannable_data: SpannableData::GreenText
      };

      if spannable.is_valid() {
        out_spannables.push(spannable);
      }
    }

    if element.has_class("deadlink") {
      // dead post quote
    }
  }

}