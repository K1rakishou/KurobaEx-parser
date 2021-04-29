use crate::rules::rule_handler::RuleHandler;
use crate::comment_parser::parser::Spannable;
use crate::html_parser::element::Element;

pub struct SpanHandler {}

impl SpanHandler {
  pub fn new() -> SpanHandler {
    return SpanHandler {};
  }
}

// <a href=\"#p333650561\" class=\"quotelink\">
//  &gt;&gt;333650561
// </a>
// <br>
// <span class=\"quote\">
//  &gt;what&#039;s the best alternative
// </span>
// <br>Reps

impl RuleHandler for SpanHandler {
  fn handle(&self, element: &Element, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) -> bool {
    // if (element.classes.contains("quote")) {
    //
    // }

    return true;
  }
}