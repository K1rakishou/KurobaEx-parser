use crate::rules::rule_handler::{RuleHandler, RuleHandlerPostHandleMeta};
use crate::comment_parser::comment_parser::{Spannable, SpannableData, PostLink};
use crate::html_parser::element::Element;
use crate::post_parser::post_parser::PostParserContext;
use crate::PostRaw;
use crate::util::helpers::SumBy;
use crate::html_parser::node::Node;
use std::num::ParseIntError;
use crate::rules::anchor::handle_single_post_quote;

const TAG: &str = "SpanHandler";

pub struct SpanHandler {}

impl RuleHandler for SpanHandler {

  fn pre_handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<String>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool {
    if element.has_class("deadlink") {
      // dead post quote
      return self.handle_deadlink_class(post_raw, post_parser_context, element, out_text_parts, out_spannables);
    }

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
    if prev_out_text_parts_index < 0  {
      // Nothing was added since handle() call. This probably means that the current tag has an empty
      // body.
      eprintln!("{} prev_out_text_parts_index < 0 ({})", TAG, prev_out_text_parts_index);
      return;
    }

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

    if element.has_class("quote") {
      // greentext
      self.handle_quote_class(prev_out_text_parts_index, out_text_parts, out_spannables)
    }

    if element.has_class("deadlink") {
      // handled in pre_handled()
      return;
    }
  }

}

impl SpanHandler {
  pub fn new() -> SpanHandler {
    return SpanHandler {};
  }

  fn handle_deadlink_class(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<String>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool {
    if element.children.len() > 1 {
      eprintln!("{} element.children.len() != 1, len={}", TAG, element.children.len() > 1);
      return true;
    }

    let quote_text_child_node = element.children.first().unwrap();

    let quote_text_child = match quote_text_child_node {
      Node::Text(link_text_child_node_text) => {
        String::from(html_escape::decode_html_entities(&link_text_child_node_text))
      }
      Node::Element(element) => {
        eprintln!("{} unexpected node: {}, expected Node::Text", TAG, element);
        return true;
      }
    };

    if quote_text_child.starts_with(">>") {
      let quote_text = &quote_text_child[2..];
      let quote_value_result = quote_text.parse::<u64>();

      let quote_value = match quote_value_result {
        Ok(value) => value,
        Err(_) => {
          eprintln!("{} failed to convert quote_text: {} into u64", TAG, quote_text);
          return true;
        }
      };

      let post_link = if post_parser_context.is_internal_thread_post(quote_value) {
        PostLink::Quote { post_no: quote_value }
      } else {
        PostLink::Dead { post_no: quote_value }
      };

      let total_text_length = out_text_parts.iter().sum_by(&|string| string.len() as i32);

      handle_single_post_quote(
        post_raw,
        post_parser_context,
        out_text_parts,
        out_spannables,
        post_link,
        &quote_text_child,
        total_text_length
      );

      return true;
    }

    eprintln!("{} Failed to parse link_text_child ({})", TAG, quote_text_child);
    return true;
  }

  fn handle_quote_class(
    &self,
    prev_out_text_parts_index: usize,
    out_text_parts: &mut Vec<String>,
    out_spannables: &mut Vec<Spannable>
  ) {
    let start = (self as &dyn RuleHandler).get_out_text_parts_diff_len(
      prev_out_text_parts_index,
      &out_text_parts
    );

    let len = (self as &dyn RuleHandler).get_out_text_parts_new_len(
      prev_out_text_parts_index,
      &out_text_parts
    ) as usize;

    let spannable = Spannable {
      start,
      len,
      spannable_data: SpannableData::GreenText
    };

    if spannable.is_valid() {
      out_spannables.push(spannable);
    }
  }
}
