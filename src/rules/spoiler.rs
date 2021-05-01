use crate::rules::rule_handler::RuleHandler;
use crate::PostRaw;
use crate::post_parser::post_parser::PostParserContext;
use crate::html_parser::element::Element;
use crate::comment_parser::comment_parser::{Spannable, SpannableData, PostLink};
use crate::html_parser::node::Node;
use crate::util::helpers::SumBy;

pub struct SpoilerHandler {}

impl SpoilerHandler {
  pub fn new() -> SpoilerHandler {
    return SpoilerHandler {};
  }
}

impl RuleHandler for SpoilerHandler {

  fn handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<String>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool {
    if element.children.is_empty() {
      return true;
    }

    if element.children.len() > 1 {
      return false;
    }

    let spoiler_text_child_node = element.children.first().unwrap();

    let spoiler_text_child = match spoiler_text_child_node {
      Node::Text(text) => text,
      Node::Element(_) => {
        println!("Unexpected child node: {}", spoiler_text_child_node);
        return false;
      }
    };

    let unescaped_text = String::from(html_escape::decode_html_entities(spoiler_text_child));
    let total_text_length = out_text_parts.sum_by(&|string| string.len() as i32);

    let spannable = Spannable {
      start: total_text_length,
      len: unescaped_text.len(),
      spannable_data: SpannableData::Link(PostLink::Spoiler)
    };

    out_spannables.push(spannable);
    out_text_parts.push(String::from(unescaped_text));

    return true;
  }

}