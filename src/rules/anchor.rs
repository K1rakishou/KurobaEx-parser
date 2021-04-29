use crate::comment_parser::parser::{Spannable, PostLink, SpannableData};
use crate::rules::rule_handler::RuleHandler;
use std::num::ParseIntError;
use crate::html_parser::element::Element;
use crate::html_parser::node::Node;

pub struct AnchorRuleHandler {}

impl AnchorRuleHandler {
  pub fn new() -> AnchorRuleHandler {
    return AnchorRuleHandler {};
  }
}

trait SumBy<T> {
  fn sumBy(&self, func: &dyn Fn(&T) -> i32) -> i32;
}

impl<T> SumBy<T> for Vec<T> {
  fn sumBy(&self, func: &dyn Fn(&T) -> i32) -> i32 {
    let mut sum: i32 = 0;

    for element in self.iter() {
      sum += func(&element);
    }

    return sum;
  }
}

impl RuleHandler for AnchorRuleHandler {
  fn handle(&self, element: &Element, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) -> bool {
    if element.children.len() > 0 {
      let link_text_child = element.children.first().unwrap();

      match link_text_child {
        Node::Text(text) => {
          let hrefValueMaybe = element.attributes.get("href");
          if hrefValueMaybe.is_some() {
            let quoteRawMaybe = hrefValueMaybe.unwrap();
            if quoteRawMaybe.is_some() {
              let quoteRaw = quoteRawMaybe.as_ref().unwrap();
              let quoteResult = quote_raw_to_quote(&quoteRaw);

              match quoteResult {
                Err(err) => {
                  println!("Failed to convert quoteRaw=\'{}\' into postNo", quoteRaw);
                }
                Ok(post_no) => {
                  let unescaped_text = String::from(html_escape::decode_html_entities(text));
                  let total_text_length = out_text_parts.sumBy(&|string| string.len() as i32);

                  let spannable = Spannable {
                    start: total_text_length,
                    len: unescaped_text.len(),
                    spannable_data: SpannableData::Link(PostLink::Quote { post_no })
                  };

                  out_spannables.push(spannable);
                  out_text_parts.push(String::from(unescaped_text));
                }
              }
            }
          }
        },
        Node::Element(_) => {}
      }
    }

    if element.children.len() > 1 {
      return false;
    }

    return true;
  }
}

fn quote_raw_to_quote(quote_raw: &str) -> Result<u64, ParseIntError> {
  if quote_raw.starts_with("#p") {
    let quote_str = &quote_raw[2..];
    return quote_str.parse::<u64>();
  }

  return quote_raw.parse::<u64>();
}