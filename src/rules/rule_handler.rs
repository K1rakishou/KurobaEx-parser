use crate::comment_parser::comment_parser::Spannable;
use crate::html_parser::element::Element;
use crate::post_parser::post_parser::PostParserContext;
use crate::PostRaw;

pub trait RuleHandler {

  fn handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<String>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool;

}