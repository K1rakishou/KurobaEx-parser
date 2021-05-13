use crate::rules::rule_handler::RuleHandler;
use crate::html_parser::node::Node;
use crate::parsing_error::ParsingError;
use regex::Regex;
use crate::{PostRaw, PostParserContext, Element, Spannable, PostLink, SpannableData, TextPart};
use crate::util::helpers::SumBy;

const TAG: &str = "AnchorRuleHandler";
const HREF: &str = "href";
const CROSS_THREAD_POSTFIX: &str = " →";
const OP_POSTFIX: &str = " (OP)";
const ME_POSTFIX: &str = " (Me)";
const YOU_POSTFIX: &str = " (You)";
const DEAD_POSTFIX: &str = " (DEAD)";

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

impl RuleHandler for AnchorRuleHandler {
  fn pre_handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<TextPart>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool {
    if element.children.len() != 1 {
      eprintln!("{} element.children.len() != 1, len={}", TAG, element.children.len() > 1);
      return false;
    }

    let link_text_child = element.children.first().unwrap();
    match link_text_child {
      Node::Text(text) => {
        handle_href_attr(element, post_raw, post_parser_context, out_text_parts, out_spannables, text)
      },
      Node::Element(element) => {
        eprintln!("{} UNKNOWN TAG: tag_name=<a>, element={}", TAG, element)
      }
    }

    return true;
  }

  fn post_handle(
    &self,
    _: &PostRaw,
    _: &PostParserContext,
    _: &Element,
    _: usize,
    _: &mut Vec<TextPart>,
    _: usize,
    _: &mut Vec<Spannable>
  ) {
    // no-op
  }

}

fn handle_href_attr<'a>(
  element: &Element,
  post_raw: &PostRaw,
  post_parser_context: &PostParserContext,
  out_text_parts: &mut Vec<TextPart>,
  out_spannables: &mut Vec<Spannable>,
  text: &String
) {
  let href_value_maybe = element.attributes.get(HREF);
  if href_value_maybe.is_none() {
    eprintln!("{} <a> tag has no \"{}\" attribute", TAG, HREF);
    return;
  }

  let link_raw = href_value_maybe.unwrap();
  let post_link_result = link_raw_to_post_link(post_parser_context, &link_raw);

  match post_link_result {
    Err(err) => {
      eprintln!("{} Failed to convert quoteRaw=\"{}\" into postNo, err={}", TAG, link_raw, err);
    }
    Ok(post_link) => {
      let unescaped_text = String::from(html_escape::decode_html_entities(text));
      let total_text_length = out_text_parts.iter().sum_by(&|string| string.characters_count as i32) as usize;

      match &post_link {
        PostLink::Quote { .. } | PostLink::Dead { .. } => {
          handle_single_post_quote(
            post_raw,
            post_parser_context,
            out_text_parts,
            out_spannables,
            post_link,
            &unescaped_text,
            total_text_length
          );
        },
        PostLink::UrlLink { .. } |
        PostLink::BoardLink { .. } |
        PostLink::SearchLink  { .. } |
        PostLink::ThreadLink { .. } => {
          let result_text = if let PostLink::ThreadLink { .. } = post_link {
            String::from(unescaped_text + CROSS_THREAD_POSTFIX)
          } else {
            unescaped_text
          };

          let result_text_part = TextPart::new(result_text);

          let spannable = Spannable {
            start: total_text_length,
            len: result_text_part.characters_count,
            spannable_data: SpannableData::Link(post_link)
          };

          if spannable.is_valid() {
            out_spannables.push(spannable);
          }

          out_text_parts.push(result_text_part);
        }
      }
    }
  }
}

pub fn handle_single_post_quote(
  post_raw: &PostRaw,
  post_parser_context: &PostParserContext,
  out_text_parts: &mut Vec<TextPart>,
  out_spannables: &mut Vec<Spannable>,
  post_link: PostLink,
  unescaped_text: &String,
  span_start: usize
) {
  let quote_post_id = match post_link {
    PostLink::Quote { post_no } => post_no,
    PostLink::Dead { post_no } => post_no,
    wrong_post_link@ PostLink::UrlLink {..} |
    wrong_post_link@ PostLink::BoardLink {..} |
    wrong_post_link@ PostLink::SearchLink {..} |
    wrong_post_link@ PostLink::ThreadLink {..} => {
      panic!("{} post_link ({}) shouldn't be handled here", TAG, wrong_post_link)
    }
  };

  let is_dead = match post_link {
    PostLink::Quote { .. } => false,
    PostLink::Dead { .. } => true,
    wrong_post_link@ PostLink::UrlLink {..} |
    wrong_post_link@ PostLink::BoardLink {..} |
    wrong_post_link@ PostLink::SearchLink {..} |
    wrong_post_link@ PostLink::ThreadLink {..} => {
      panic!("{} post_link ({}) shouldn't be handled here", TAG, wrong_post_link)
    }
  };

  let mut quote_text_suffixes = String::new();

  if post_raw.is_quoting_original_post(quote_post_id) {
    quote_text_suffixes.push_str(OP_POSTFIX);
  }

  if post_parser_context.is_my_reply_to_my_own_post(post_raw.post_no(), quote_post_id) {
    quote_text_suffixes.push_str(ME_POSTFIX);
  } else if post_parser_context.is_reply_to_my_post(quote_post_id) {
    quote_text_suffixes.push_str(YOU_POSTFIX);
  }

  if is_dead {
    quote_text_suffixes.push_str(DEAD_POSTFIX);
  }

  let quote_text_result = format!("{}{}", String::from(unescaped_text), quote_text_suffixes);

  let spannable = Spannable {
    start: span_start,
    len: quote_text_result.len(),
    spannable_data: SpannableData::Link(post_link)
  };

  if spannable.is_valid() {
    out_spannables.push(spannable);
  }

  out_text_parts.push(TextPart::new(quote_text_result));
}

fn link_raw_to_post_link(
  post_parser_context: &PostParserContext,
  link_raw: &str
) -> Result<PostLink, ParsingError> {
  if link_raw.starts_with("#p") {
    // Normal in-thread post quote: "#p333790203"
    let quote_str = &link_raw[2..];
    let post_no = quote_str.parse::<u64>().unwrap();

    return if post_parser_context.is_internal_thread_post(post_no) {
      Result::Ok(PostLink::Quote { post_no })
    } else {
      Result::Ok(PostLink::Dead { post_no })
    }
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

  let thread_link = PostLink::UrlLink {
    link: String::from(link_raw)
  };

  return Result::Ok(thread_link);
}