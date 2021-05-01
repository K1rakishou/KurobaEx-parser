use crate::rules::rule_handler::RuleHandler;
use crate::comment_parser::comment_parser::Spannable;
use crate::html_parser::element::Element;
use crate::post_parser::post_parser::PostParserContext;
use crate::PostRaw;

pub struct LineBreakRuleHandler {}

impl LineBreakRuleHandler {
  pub fn new() -> LineBreakRuleHandler {
    return LineBreakRuleHandler {};
  }
}

impl RuleHandler for LineBreakRuleHandler {
  fn handle(&self, _: &PostRaw,_: &PostParserContext, _: &Element, out_text_parts: &mut Vec<String>, _: &mut Vec<Spannable>) -> bool {
    out_text_parts.push(String::from('\n'));
    return true;
  }
}