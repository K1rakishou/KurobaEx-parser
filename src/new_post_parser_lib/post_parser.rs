pub mod post_parser {
  use crate::{PostRaw, PostParserContext, PostParser, CommentParser, ParsedPost, ParsedSpannableText, Spannable, SpannableData, PostLink, HtmlParser, TextPart};
  use crate::html_parser::node::Node;
  use std::collections::HashSet;
  use std::fmt;
  use regex::Regex;
  use crate::util::helpers::{SumBy, MapJoin};

  lazy_static! {
    static ref LINK_PATTERN: Regex = Regex::new(r"https?://[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-\w0-9()@:%_\+.~#?&//=]*)").unwrap();
  }

  impl PostParserContext {
    pub fn new(
      site_name: &str,
      board_code: &str,
      thread_id: u64,
      my_replies: HashSet<u64>,
      thread_posts: HashSet<u64>
    ) -> PostParserContext {
      return PostParserContext {
        site_name: String::from(site_name),
        board_code: String::from(board_code),
        thread_id,
        my_replies,
        thread_posts
      }
    }

    pub fn is_internal_thread_post(&self, quote_post_id: u64) -> bool {
      return self.thread_posts.contains(&quote_post_id);
    }

    pub fn is_reply_to_my_post(&self, quote_post_id: u64) -> bool {
      return self.my_replies.contains(&quote_post_id);
    }

    pub fn is_my_reply_to_my_own_post(&self, source_post_id: u64, quote_post_id: u64) -> bool {
      return self.my_replies.contains(&source_post_id) && self.my_replies.contains(&quote_post_id);
    }

    pub fn is_quoting_original_post(&self, quote_post_id: u64) -> bool {
      return self.thread_id == quote_post_id;
    }

  }

  impl fmt::Display for ParsedPost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      return write!(f, "ParsedPost(post_comment_parsed: {}", &self.post_comment_parsed);
    }
  }

  impl ParsedPost {
    pub fn new(
      post_parser_context: &PostParserContext,
      post_id: u64,
      post_sub_id: u64,
      post_comment_parsed: ParsedSpannableText
    ) -> ParsedPost {
      ParsedPost {
        site_name: post_parser_context.site_name.clone(),
        board_code: post_parser_context.board_code.clone(),
        thread_id: post_parser_context.thread_id,
        post_id,
        post_sub_id,
        post_comment_parsed
      }
    }
  }

  impl PostParser<'_> {
    pub fn new(post_parser_context: &PostParserContext) -> PostParser {
      let mut comment_parser = CommentParser::new(post_parser_context);

      comment_parser.add_default_matching_rules();
      comment_parser.add_replacement_rule("<wbr>", "");

      return PostParser {
        post_parser_context,
        comment_parser: Box::new(comment_parser)
      };
    }

    pub fn parse_post(&self, post_raw: &PostRaw) -> ParsedPost {
      return ParsedPost::new(
        self.post_parser_context,
        post_raw.post_id,
        post_raw.post_sub_id,
        self.parse_comment(post_raw)
      )
    }

    pub fn iterate_comment_nodes(&self, post_comment: &String, iterator: &dyn Fn(&Node, &String)) {
      let html_parser = HtmlParser::new();
      let html_parsing_result = html_parser.parse(post_comment);

      if html_parsing_result.is_err() {
        return;
      }

      self.iterate_element(&html_parsing_result.unwrap(), post_comment, iterator);
    }

    pub fn parse_comment(&self, post_raw: &PostRaw) -> ParsedSpannableText {
      let comment_raw = self.pre_process_comment(post_raw);
      if comment_raw.is_empty() {
        return ParsedSpannableText::empty();
      }

      let html_parser = HtmlParser::new();

      let html_parsing_result = html_parser.parse(comment_raw.as_str());
      if html_parsing_result.is_err() {
        let parser_error_message = format!(
          "Failed to parse comment_raw html, error={:?}",
          html_parsing_result.err()
        );

        let post_comment_parsed = ParsedSpannableText::new(
          comment_raw.as_str(),
          Box::new(parser_error_message),
          Box::new(Vec::new())
        );

        return post_comment_parsed;
      }

      let mut out_text_parts: Vec<TextPart> = Vec::with_capacity(16);
      let mut out_spannables: Vec<Spannable> = Vec::with_capacity(8);
      self.process_element(post_raw, &html_parsing_result.unwrap(), &mut out_text_parts, &mut out_spannables);

      let total_size = out_text_parts.iter().sum_by(&|text_part| text_part.characters_count as i32) as usize;

      return ParsedSpannableText::new(
        comment_raw.as_str(),
        Box::new(out_text_parts.iter().map_join_cap(total_size, "", &|text_part| text_part.text.as_str())),
        Box::new(out_spannables)
      );
    }

    fn pre_process_comment(&self, post_raw: &PostRaw) -> String {
      let comment_raw = &post_raw.com;
      if comment_raw.is_empty() {
        return String::from("");
      }

      let mut result_comment_raw = String::from(comment_raw);

      for (pattern, value) in &self.comment_parser.replacement_rules {
        result_comment_raw = result_comment_raw.replace(pattern, &value);
      }

      return result_comment_raw;
    }

    fn process_element(
      &self,
      post_raw: &PostRaw,
      nodes: &Vec<Node>,
      out_text_parts: &mut Vec<TextPart>,
      out_spannables: &mut Vec<Spannable>
    ) {
      for node in nodes {
        match node {
          Node::Text(text) => {
            let unescaped_text = String::from(html_escape::decode_html_entities(text.as_str()));
            self.detect_links(out_text_parts, &unescaped_text, out_spannables);

            out_text_parts.push(TextPart::new(unescaped_text));
          },
          Node::Element(element) => {
            // store the current last indexes of out_text_parts/out_spannables because we may need
            // them during post process phase to figure out what was added into
            // out_text_parts/out_spannables
            let prev_out_text_parts_index = out_text_parts.len().checked_sub(1).unwrap_or(0);
            let prev_out_spannables_index = out_spannables.len().checked_sub(1).unwrap_or(0);

            if self.comment_parser.pre_process_element(post_raw, &element, out_text_parts, out_spannables) {
              // Element was fully processed, no need to check the child elements
              continue;
            }

            if !element.children.is_empty() {
              self.process_element(post_raw, &element.children, out_text_parts, out_spannables);

              self.comment_parser.post_process_element(
                post_raw,
                &element,
                prev_out_text_parts_index,
                out_text_parts,
                prev_out_spannables_index,
                out_spannables
              )
            }
          },
        }
      }
    }

    fn iterate_element(
      &self,
      nodes: &Vec<Node>,
      post_comment: &String,
      iterator: &dyn Fn(&Node, &String)
    ) {
      for node in nodes {
        match node {
          Node::Text(_) => {
            iterator(&node, post_comment);
          },
          Node::Element(element) => {
            iterator(&node, post_comment);

            if !element.children.is_empty() {
              self.iterate_element(&element.children, &post_comment, &iterator);
            }
          },
        }
      }
    }

    pub fn detect_links(&self, out_text_parts: &mut Vec<TextPart>, text: &String, out_spannables: &mut Vec<Spannable>) {
      let mut capture_locations = LINK_PATTERN.capture_locations();
      let mut offset: usize = 0;

      let total_text_chars_count = out_text_parts
        .iter()
        .sum_by(&|string| string.characters_count as i32) as usize;

      let text_bytes = text.as_bytes();

      loop {
        LINK_PATTERN.captures_read_at(&mut capture_locations, text, offset);

        let capture_maybe = capture_locations.get(0);
        if capture_maybe.is_none() {
          break;
        }

        let (bytes_capture_start, bytes_capture_end) = capture_maybe.unwrap();
        if bytes_capture_start >= bytes_capture_end || bytes_capture_end <= 0 {
          break;
        }

        let actual_link = String::from_utf8_lossy(&text_bytes[bytes_capture_start..bytes_capture_end]);

        // [bytes_capture_start] and [bytes_capture_end] do not account unicode characters being
        // present in [text]. If there are then they will be incorrect. To fix that we need to count
        // the amount of unicode characters between [0..bytes_capture_start] and
        // [bytes_capture_start..bytes_capture_end]. This is slow and uses memory but I have no idea how to do it
        // correctly.

        let start = String::from_utf8_lossy(&text.as_bytes()[0..bytes_capture_start]).chars().count();
        let len = String::from_utf8_lossy(&text.as_bytes()[bytes_capture_start..bytes_capture_end]).chars().count();

        let link_spannable = Spannable {
          start: total_text_chars_count + start,
          len,
          spannable_data: SpannableData::Link(PostLink::UrlLink { link: actual_link.to_string() })
        };

        if link_spannable.is_valid() {
          out_spannables.push(link_spannable);
        }

        offset = bytes_capture_end;
      }
    }

  }
}