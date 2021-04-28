pub mod parser {
  use html_parser::Dom;
  use html_parser::Node;
  use html_parser::Element;
  use std::collections::HashMap;
  use crate::comment_parser::parser::{PostCommentParsed, Spannable, CommentParser};

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

    pub fn parse_comment(&self, comment_raw: &str) -> Option<PostCommentParsed> {
      let parseResult = Dom::parse(comment_raw);
      if (parseResult.is_err()) {
        println!("Failed to parse comment_raw html, error={:?}", parseResult.err().unwrap());
        return Option::None;
      }

      let dom = parseResult.unwrap();

      let mut outTextParts: Vec<String> = Vec::new();
      let mut outSpannables: Vec<Spannable> = Vec::new();
      self.parse_nodes(dom.children, &mut outTextParts, &mut outSpannables);

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
          Node::Comment(_) => continue,
        }
      }
    }

  }

}