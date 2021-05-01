pub mod comment_parser {
  use std::collections::HashMap;

  use crate::rules::anchor::AnchorRuleHandler;
  use crate::rules::span::SpanHandler;
  use crate::rules::rule_handler::RuleHandler;
  use crate::rules::line_break::LineBreakRuleHandler;
  use crate::html_parser::element::Element;
  use crate::rules::word_break::WordBreakRuleHandler;
  use crate::post_parser::post_parser::PostParserContext;
  use std::fmt;
  use std::fmt::Formatter;
  use crate::PostRaw;

  #[derive(Debug, PartialEq)]
  pub enum PostLink {
    Quote { post_no: u64 },
    Dead { post_no: u64 },
    UrlLink { link: String },
    BoardLink { board_code: String },
    SearchLink { board_code: String, search_query: String },
    ThreadLink { board_code: String, thread_no: u64, post_no: u64 }
  }

  impl fmt::Display for PostLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      return match self {
        PostLink::Quote { post_no } => {
          write!(f, "Quote(post_no: {})", post_no)
        },
        PostLink::Dead { post_no } => {
          write!(f, "Dead(post_no: {})", post_no)
        },
        PostLink::UrlLink { link } => {
          write!(f, "UrlLink(link: {})", link)
        },
        PostLink::BoardLink { board_code } => {
          write!(f, "BoardLink(board_code: {})", board_code)
        },
        PostLink::SearchLink { board_code, search_query } => {
          write!(f, "SearchLink(board_code: {}, search_query: {})", board_code, search_query)
        },
        PostLink::ThreadLink { board_code, thread_no, post_no } => {
          write!(f, "ThreadLink(board_code: {}, thread_no: {}, post_no: {})", board_code, thread_no, post_no)
        }
      }
    }
  }

  #[derive(Debug, PartialEq)]
  pub enum SpannableData {
    Link(PostLink)
  }

  impl fmt::Display for SpannableData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      return match self {
        SpannableData::Link(post_link) => {
          write!(f, "PostLink(post_link: {})", post_link)
        }
      }
    }
  }

  #[derive(Debug, PartialEq)]
  pub struct Spannable {
    pub start: i32,
    pub len: usize,
    pub spannable_data: SpannableData
  }

  impl fmt::Display for Spannable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Spannable(start: {}, len: {}, spannable_data: {})", self.start, self.len, self.spannable_data)
    }
  }

  pub struct PostCommentParsed {
    pub comment_text: Box<String>,
    pub spannables: Box<Vec<Spannable>>
  }

  impl fmt::Display for PostCommentParsed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "PostCommentParsed(comment_text: {}", self.comment_text).unwrap();

      for spannable in self.spannables.iter() {
        write!(f, ", spannable: {}", spannable).unwrap();
      }

      return write!(f, ")");
    }
  }

  impl PostCommentParsed {
    pub fn new(comment_text: Box<String>, spannables: Box<Vec<Spannable>>) -> PostCommentParsed {
      PostCommentParsed {
        comment_text,
        spannables
      }
    }
  }

  enum ParsingRule {
    CustomRule(Box<dyn RuleHandler>)
  }

  pub struct CommentParser<'a> {
    post_parser_context: &'a PostParserContext,
    rules: HashMap<String, ParsingRule>
  }

  impl CommentParser<'_> {

    pub fn new<'a>(post_parser_context: &'a PostParserContext) -> CommentParser<'a> {
      return CommentParser {
        post_parser_context,
        rules: HashMap::new()
      }
    }

    pub fn add_default_rules(&mut self) {
      self.rules.insert(String::from("a"), ParsingRule::CustomRule(Box::new(AnchorRuleHandler::new())));
      self.rules.insert(String::from("br"), ParsingRule::CustomRule(Box::new(LineBreakRuleHandler::new())));
      self.rules.insert(String::from("wbr"), ParsingRule::CustomRule(Box::new(WordBreakRuleHandler::new())));
      self.rules.insert(String::from("span"), ParsingRule::CustomRule(Box::new(SpanHandler::new())));
    }

    pub fn process_element(
      &self,
      post_raw: &PostRaw,
      element: &Element,
      out_text_parts: &mut Vec<String>,
      out_spannables: &mut Vec<Spannable>
    ) -> bool {
      let element_name = element.name.as_str();
      let rule_maybe = self.rules.get(element_name);

      let rule = match rule_maybe {
        None => panic!("No rule found for element with name \'{}\'", element_name),
        Some(_) => rule_maybe.unwrap()
      };

      return match rule {
        ParsingRule::CustomRule(rule_handler) => {
          rule_handler.handle(post_raw, self.post_parser_context, element, out_text_parts, out_spannables)
        }
      }
    }

  }
}