use crate::rules::rule_handler::RuleHandler;
use crate::comment_parser::comment_parser::Spannable;
use crate::html_parser::element::Element;
use crate::post_parser::post_parser::PostParserContext;
use crate::PostRaw;

pub struct SpanHandler {}

impl SpanHandler {
  pub fn new() -> SpanHandler {
    return SpanHandler {};
  }
}

impl RuleHandler for SpanHandler {

  fn handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<String>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool {
    // if (element.classes.contains("quote")) {
    //
    // }

    return true;
  }

}