use crate::rules::rule_handler::RuleHandler;
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

  fn handle(
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

    if prev_out_text_parts_index > 0
      && prev_out_text_parts_index == out_text_parts.len()
      && prev_out_spannables_index == out_spannables.len()
    {
      // Nothing was added since handle() call so we apparently have nothing to do? Or maybe we do have?
      return;
    }

    if element.has_class("quote") {
      // greentext
      let start = out_text_parts[0..prev_out_text_parts_index]
        .iter()
        .sum_by(&|string| string.len() as i32);

      let len = out_text_parts[prev_out_text_parts_index..]
        .iter()
        .sum_by(&|string| string.len() as i32);

      let spannable = Spannable {
        start,
        len: len as usize,
        spannable_data: SpannableData::GreenText
      };

      if spannable.is_valid() {
        out_spannables.push(spannable);
      }
    }

    if element.has_class("deadlink") {
      // dead post quote
      println!("deadlink")
    }
  }

}