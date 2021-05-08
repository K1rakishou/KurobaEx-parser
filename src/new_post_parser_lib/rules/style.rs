use crate::rules::rule_handler::{RuleHandler, RuleHandlerPostHandleMeta};
use crate::{PostRaw, PostParserContext, Element, Spannable, TextPart};
use crate::util::style_tag_value_decoder::decode_style_spans;

const TAG: &str = "StyleHandler";

pub struct StyleHandler {}

impl StyleHandler {
  pub fn new() -> StyleHandler {
    return StyleHandler {};
  }
}

impl RuleHandler for StyleHandler {

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
    element: &Element,
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

    let style_attr_value_maybe = element.get_attr_value("style");

    let style_attr_value = if let Option::None = style_attr_value_maybe {
      return;
    } else {
      style_attr_value_maybe.unwrap()
    };

    let start = (self as &dyn RuleHandler).get_out_text_parts_diff_len(
      prev_out_text_parts_index,
      &out_text_parts
    ) as usize;

    let len = (self as &dyn RuleHandler).get_out_text_parts_new_len(
      prev_out_text_parts_index,
      &out_text_parts
    ) as usize;

    let spannables = decode_style_spans(style_attr_value)
      .iter()
      .map(|spannable_data| {
        return Spannable {
          start: start as usize,
          len: len as usize,
          spannable_data: spannable_data.clone()
        };
      }).collect::<Vec<Spannable>>();

    for spannable in spannables {
      if spannable.is_valid() {
        out_spannables.push(spannable);
      }
    }
  }

}