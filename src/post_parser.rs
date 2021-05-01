pub mod post_parser {
  use crate::comment_parser::comment_parser::{PostCommentParsed, Spannable, CommentParser};
  use crate::PostRaw;
  use crate::html_parser::node::Node;
  use crate::html_parser::parser::HtmlParser;
  use std::collections::HashSet;
  use std::fmt;

  #[derive(Debug)]
  pub struct PostParserContext {
    thread_id: u64,
    my_replies: HashSet<u64>,
    thread_posts: HashSet<u64>
  }

  impl PostParserContext {
    pub fn new(
      thread_id: u64,
      my_replies: HashSet<u64>,
      thread_posts: HashSet<u64>
    ) -> PostParserContext {
      return PostParserContext {
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

  pub struct ParsedPost {
    pub post_comment_parsed: Option<PostCommentParsed>,
  }

  impl fmt::Display for ParsedPost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      return match &self.post_comment_parsed {
        None => {
          write!(f, "ParsedPost(post_comment_parsed: None")
        }
        Some(value) => {
          write!(f, "ParsedPost(post_comment_parsed: {}", value)
        }
      };
    }
  }

  impl ParsedPost {
    pub fn new(post_comment_parsed: Option<PostCommentParsed>) -> ParsedPost {
      ParsedPost {
        post_comment_parsed
      }
    }
  }

  pub struct PostParser<'a> {
    post_parser_context: &'a PostParserContext,
    comment_parser: Box<CommentParser<'a>>,
  }

  impl PostParser<'_> {
    pub fn new<'a>(post_parser_context: &'a PostParserContext) -> PostParser<'a> {
      let mut comment_parser = CommentParser::new(post_parser_context);
      comment_parser.add_default_rules();

      return PostParser {
        post_parser_context,
        comment_parser: Box::new(comment_parser)
      };
    }

    pub fn parse_post(&self, post_raw: &PostRaw) -> ParsedPost {
      let mut post = ParsedPost::new(Option::None);

      let comment = post_raw.com.as_ref();
      if comment.is_some() {
        post.post_comment_parsed = self.parse_comment(comment.unwrap().as_str(), post_raw);
      }

      return post
    }

    pub fn parse_comment(&self, comment_raw: &str, post_raw: &PostRaw) -> Option<PostCommentParsed> {
      let html_parser = HtmlParser::new();

      let html_parsing_result = html_parser.parse(comment_raw);
      if html_parsing_result.is_err() {
        println!("Failed to parse comment_raw html, error={:?}", html_parsing_result.err().unwrap());
        return Option::None;
      }

      let mut out_text_parts: Vec<String> = Vec::new();
      let mut out_spannables: Vec<Spannable> = Vec::new();
      self.parse_nodes(post_raw, &html_parsing_result.unwrap(), &mut out_text_parts, &mut out_spannables);

      let post_comment_parsed = PostCommentParsed::new(
        Box::new(out_text_parts.join("")),
        Box::new(out_spannables)
      );

      return Option::Some(post_comment_parsed);
    }

    fn parse_nodes(
      &self,
      post_raw: &PostRaw,
      nodes: &Vec<Node>,
      out_text_parts: &mut Vec<String>,
      out_spannables: &mut Vec<Spannable>
    ) {
      for node in nodes {
        match node {
          Node::Text(text) => {
            let unescaped_text = String::from(html_escape::decode_html_entities(text.as_str()));
            out_text_parts.push(unescaped_text);
          },
          Node::Element(element) => {
            // store the current last indexes of out_text_parts/out_spannables because we may need
            // them during post process phase to figure out what was added into
            // out_text_parts/out_spannables
            let prev_out_text_parts_index = out_text_parts.len().checked_sub(1).unwrap_or(0);
            let prev_out_spannables_index = out_spannables.len().checked_sub(1).unwrap_or(0);

            if self.comment_parser.process_element(post_raw, &element, out_text_parts, out_spannables) {
              continue;
            }

            if !element.children.is_empty() {
              self.parse_nodes(post_raw, &element.children, out_text_parts, out_spannables);
            }

            self.comment_parser.post_process_element(
              post_raw,
              &element,
              prev_out_text_parts_index,
              out_text_parts,
              prev_out_spannables_index,
              out_spannables
            )
          },
        }
      }
    }

  }

}