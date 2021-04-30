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
  fn sum_by(&self, func: &dyn Fn(&T) -> i32) -> i32;
}

impl<T> SumBy<T> for Vec<T> {
  fn sum_by(&self, func: &dyn Fn(&T) -> i32) -> i32 {
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
          let href_value_maybe = element.attributes.get("href");
          if href_value_maybe.is_some() {
            let quote_raw_maybe = href_value_maybe.unwrap();
            if quote_raw_maybe.is_some() {
              let quote_raw = quote_raw_maybe.as_ref().unwrap();
              let quote_result = quote_raw_to_quote(&quote_raw);

              match quote_result {
                Err(err) => {
                  println!("Failed to convert quoteRaw=\'{}\' into postNo, err={}", quote_raw, err);
                }
                Ok(post_no) => {
                  let unescaped_text = String::from(html_escape::decode_html_entities(text));
                  let total_text_length = out_text_parts.sum_by(&|string| string.len() as i32);

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