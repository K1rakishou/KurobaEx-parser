pub mod parser {
  use std::collections::HashMap;
  use crate::comment_parser::parser::{PostCommentParsed, Spannable, CommentParser};
  use crate::PostRaw;
  use crate::html_parser::node::Node;
  use crate::html_parser::parser::HtmlParser;

  #[derive(Debug)]
  pub struct ParsedPost {
    pub post_comment_parsed: Option<PostCommentParsed>,
  }

  impl ParsedPost {
    pub fn new(post_comment_parsed: Option<PostCommentParsed>) -> ParsedPost {
      ParsedPost {
        post_comment_parsed
      }
    }
  }

  pub struct PostParser {
    comment_parser: Box<CommentParser>,
  }

  impl PostParser {
    pub fn new() -> PostParser {
      let mut comment_parser = CommentParser::new();
      comment_parser.add_default_rules();

      return PostParser { comment_parser: Box::new(comment_parser) };
    }

    pub fn parse_post(&self, post_raw: &PostRaw) -> ParsedPost {
      let mut post = ParsedPost::new(Option::None);

      let comment = post_raw.com.as_ref();
      if comment.is_some() {
        post.post_comment_parsed = self.parse_comment(comment.unwrap().as_str());
      }

      return post
    }

    pub fn parse_comment(&self, comment_raw: &str) -> Option<PostCommentParsed> {
      let html_parser = HtmlParser::new();

      let html_parsing_result = html_parser.parse(comment_raw);
      if html_parsing_result.is_err() {
        println!("Failed to parse comment_raw html, error={:?}", html_parsing_result.err().unwrap());
        return Option::None;
      }

      let mut outTextParts: Vec<String> = Vec::new();
      let mut outSpannables: Vec<Spannable> = Vec::new();
      self.parse_nodes(html_parsing_result.unwrap(), &mut outTextParts, &mut outSpannables);

      let postCommentParsed = PostCommentParsed::new(
        Box::new(outTextParts.join("")),
        Box::new(outSpannables)
      );

      return Option::Some(postCommentParsed);
    }

    fn parse_nodes(&self, nodes: Vec<Node>, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) {
      for node in nodes {
        match node {
          Node::Text(text) => {
            let unescaped_text = String::from(html_escape::decode_html_entities(text.as_str()));
            out_text_parts.push(unescaped_text);
          },
          Node::Element(element) => {
            if self.comment_parser.process_element(&element, out_text_parts, out_spannables) {
              continue;
            }

            if !element.children.is_empty() {
              self.parse_nodes(element.children, out_text_parts, out_spannables);
            }
          },
        }
      }
    }

  }

}