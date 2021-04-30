use crate::comment_parser::parser::{Spannable, PostLink, SpannableData};
use crate::rules::rule_handler::RuleHandler;
use crate::html_parser::element::Element;
use crate::html_parser::node::Node;
use crate::parsing_error::ParsingError;
use std::ops::Index;
use regex::Regex;
use serde::de::Unexpected::Str;

lazy_static! {
  static ref BOARD_LINK_PATTERN: Regex = Regex::new(r"//.*/(\w+)/$").unwrap();
  static ref BOARD_LINK_WITH_SEARCH_PATTERN: Regex = Regex::new(r"//.*/(\w+)/catalog#s=(\w+)$").unwrap();
  static ref CROSS_THREAD_LINK_PATTERN: Regex = Regex::new(r"/(\w+)/\w+/(\d+)#p(\d+)$").unwrap();
}

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
    if element.children.is_empty() {
      return true;
    }

    if element.children.len() > 1 {
      return false;
    }

    let link_text_child = element.children.first().unwrap();
    match link_text_child {
      Node::Text(text) => {
        handle_href_attr(element, out_text_parts, out_spannables, text)
      },
      Node::Element(element) => {
        println!("UNKNOWN TAG: tag_name=<a>, element={:?}", element)
      }
    }

    return true;
  }
}

fn handle_href_attr(
  element: &Element,
  out_text_parts: &mut Vec<String>,
  out_spannables: &mut Vec<Spannable>,
  text: &String
) {
  let href_value_maybe = element.attributes.get("href");
  if href_value_maybe.is_none() {
    return;
  }

  let link_raw_maybe = href_value_maybe.unwrap();
  if link_raw_maybe.is_none() {
    return;
  }

  let link_raw = link_raw_maybe.as_ref().unwrap();
  let post_link_result = link_raw_to_post_link(&link_raw);

  match post_link_result {
    Err(err) => {
      println!("Failed to convert quoteRaw=\'{}\' into postNo, err={}", link_raw, err);
    }
    Ok(post_link) => {
      let unescaped_text = String::from(html_escape::decode_html_entities(text));
      let total_text_length = out_text_parts.sum_by(&|string| string.len() as i32);

      let spannable = Spannable {
        start: total_text_length,
        len: unescaped_text.len(),
        spannable_data: SpannableData::Link(post_link)
      };

      out_spannables.push(spannable);
      out_text_parts.push(String::from(unescaped_text));
    }
  }
}

fn link_raw_to_post_link(link_raw: &str) -> Result<PostLink, ParsingError> {
  if link_raw.starts_with("#p") {
    // Normal in-thread post quote: "#p333790203"
    let quote_str = &link_raw[2..];
    let post_no = quote_str.parse::<u64>().unwrap();

    return Result::Ok(PostLink::Quote { post_no });
  }

  if link_raw.starts_with("//") {
    // Board link: "//boards.4channel.org/jp/"
    let board_link_captures_maybe = BOARD_LINK_PATTERN.captures(link_raw);
    if board_link_captures_maybe.is_some() {
      let captures = board_link_captures_maybe.unwrap();

      let board_code_maybe = captures.get(1);
      if board_code_maybe.is_some() {
        let board_code = board_code_maybe.unwrap().as_str();

        let board_link = PostLink::BoardLink {
          board_code: String::from(board_code)
        };

        return Result::Ok(board_link)
      }

      // Fallthrough
    }

    // Board link with search: "//boards.4channel.org/g/catalog#s=fglt"
    let board_link_search_captures_maybe = BOARD_LINK_WITH_SEARCH_PATTERN.captures(link_raw);
    if board_link_search_captures_maybe.is_some() {
      let captures = board_link_search_captures_maybe.unwrap();

      let board_code_maybe = captures.get(1);
      let search_query_maybe = captures.get(2);

      if board_code_maybe.is_some() && search_query_maybe.is_some() {
        let board_code = board_code_maybe.unwrap().as_str();
        let search_query = search_query_maybe.unwrap().as_str();

        let search_link = PostLink::SearchLink {
          board_code: String::from(board_code),
          search_query: String::from(search_query)
        };

        return Result::Ok(search_link)
      }

      // Fallthrough
    }
  }

  if link_raw.starts_with("/") {
    // Cross-thread link: "/vg/thread/333581281#p333581281"
    let cross_thread_link_captures_maybe = CROSS_THREAD_LINK_PATTERN.captures(link_raw);
    if cross_thread_link_captures_maybe.is_some() {
      let captures = cross_thread_link_captures_maybe.unwrap();

      let board_code_maybe = captures.get(1);
      let thread_no_str_maybe = captures.get(2);
      let post_no_str_maybe = captures.get(3);

      if board_code_maybe.is_some() && thread_no_str_maybe.is_some() && post_no_str_maybe.is_some() {
        let board_code = board_code_maybe.unwrap().as_str();
        let thread_no_str = thread_no_str_maybe.unwrap().as_str();
        let post_no_str = post_no_str_maybe.unwrap().as_str();

        let thread_no_result = thread_no_str.parse::<u64>();
        let post_no_result = post_no_str.parse::<u64>();

        if thread_no_result.is_ok() && post_no_result.is_ok() {
          let thread_no = thread_no_result.unwrap();
          let post_no = post_no_result.unwrap();

          let thread_link = PostLink::ThreadLink {
            board_code: String::from(board_code),
            thread_no,
            post_no
          };

          return Result::Ok(thread_link);
        }

        // Fallthrough
      }
    }
  }

  let full_msg = format!("Failed to parse link_raw: {}", link_raw);
  return Result::Err(ParsingError::new(&full_msg));
}