pub mod post_parser {
  use crate::{PostRaw, PostParserContext, PostParser, CommentParser, ParsedPost, ParsedSpannableText, Spannable, SpannableData, PostLink, HtmlParser, TextPart};
  use crate::html_parser::node::Node;
  use std::collections::HashSet;
  use std::fmt;
  use regex::Regex;
  use crate::util::helpers::{SumBy, MapJoin};

  lazy_static! {
    static ref CRUDE_LINK_PATTERN: Regex = Regex::new(r"(https?://(?:[^\s]+).)").unwrap();
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
      comment_parser.add_default_rules();

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

    pub fn parse_comment(&self, post_raw: &PostRaw) -> ParsedSpannableText {
      let comment_raw = &post_raw.com;
      if comment_raw.is_empty() {
        return ParsedSpannableText::empty();
      }

      let html_parser = HtmlParser::new();

      let html_parsing_result = html_parser.parse(comment_raw);
      if html_parsing_result.is_err() {
        let parser_error_message = format!(
          "Failed to parse comment_raw html, error={:?}",
          html_parsing_result.err()
        );

        let post_comment_parsed = ParsedSpannableText::new(
          comment_raw,
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
        comment_raw,
        Box::new(out_text_parts.iter().map_join_cap(total_size, "", &|text_part| text_part.text.as_str())),
        Box::new(out_spannables)
      );
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

    pub fn detect_links(&self, out_text_parts: &mut Vec<TextPart>, text: &String, out_spannables: &mut Vec<Spannable>) {
      let mut capture_locations = CRUDE_LINK_PATTERN.capture_locations();
      let mut offset: usize = 0;

      let bytes = text.as_bytes();
      let total_text_length = out_text_parts.iter().sum_by(&|string| string.characters_count as i32) as usize;

      loop {
        CRUDE_LINK_PATTERN.captures_read_at(&mut capture_locations, text, offset);

        let capture_maybe = capture_locations.get(0);
        if capture_maybe.is_none() {
          break;
        }

        let (capture_start, capture_end) = capture_maybe.unwrap();
        if capture_start >= capture_end || capture_end <= 0 {
          break;
        }

        let left_pointer = self.look_for_first_white_space_to_the_left(&bytes, capture_start);
        let right_pointer = self.look_for_first_white_space_to_the_right(&bytes, capture_end - 1);

        let actual_link = String::from(&text[left_pointer..right_pointer]);

        let link_spannable = Spannable {
          start: total_text_length + left_pointer,
          len: (right_pointer - left_pointer),
          spannable_data: SpannableData::Link(PostLink::UrlLink { link: actual_link })
        };

        if link_spannable.is_valid() {
          out_spannables.push(link_spannable);
        }

        offset = capture_end;
      }
    }

    fn look_for_first_white_space_to_the_right(&self, bytes: &[u8], capture_end: usize) -> usize {
      let mut right_pointer = capture_end;
      let bytes_length = bytes.len();

      if right_pointer == bytes_length {
        let char_maybe = bytes.get(right_pointer);
        if char_maybe.is_some() {
          let char = char_maybe.unwrap();
          if char.is_ascii_whitespace() {
            right_pointer = right_pointer.checked_sub(1).unwrap();
          }
        }

        return right_pointer;
      }

      while right_pointer < bytes_length {
        let char_maybe = bytes.get(right_pointer);
        if char_maybe.is_none() {
          break
        }

        let char = char_maybe.unwrap();
        if char.is_ascii_whitespace() {
          break
        }

        if right_pointer == bytes_length {
          break;
        }

        right_pointer = right_pointer.checked_add(1).unwrap_or(bytes_length);
      }

      return right_pointer
    }

    fn look_for_first_white_space_to_the_left(&self, bytes: &[u8], capture_start: usize) -> usize {
      let mut left_pointer = capture_start;

      if left_pointer == 0 {
        let char_maybe = bytes.get(left_pointer);
        if char_maybe.is_some() {
          let char = char_maybe.unwrap();
          if char.is_ascii_whitespace() {
            left_pointer = left_pointer.checked_add(1).unwrap();
          }
        }

        return left_pointer;
      }

      while left_pointer > 0 {
        let char_maybe = bytes.get(left_pointer);
        if char_maybe.is_none() {
          break
        }

        let char = char_maybe.unwrap();
        if char.is_ascii_whitespace() {
          left_pointer = left_pointer.checked_add(1).unwrap();
          break
        }

        if left_pointer == 0 {
          break;
        }

        left_pointer = left_pointer.checked_sub(1).unwrap_or(0);
      }

      return left_pointer;
    }

  }
}