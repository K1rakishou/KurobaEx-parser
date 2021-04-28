pub mod parser {
  use html_parser::Dom;
  use html_parser::Node;
  use html_parser::Element;
  use std::collections::HashMap;

  use crate::comment_parser::parser::ParsingRule::CustomRule;
  use crate::rules::anchor::AnchorRuleHandler;
  use crate::rules::rule_handler::RuleHandler;
  use crate::rules::line_break::LineBreakRuleHandler;

  #[derive(Debug)]
  pub enum PostLink {
    Quote { post_no: u64 },
    ThreadLink { site_name: String, board_code: String, thread_no: u64, post_no: u64 }
  }

  #[derive(Debug)]
  pub enum SpannableData {
    Link(PostLink)
  }

  #[derive(Debug)]
  pub struct Spannable {
    pub start: i32,
    pub len: usize,
    pub spannable_data: SpannableData
  }

  #[derive(Debug)]
  pub struct PostCommentParsed {
    pub comment_text: Box<String>,
    pub spannables: Box<Vec<Spannable>>
  }

  impl PostCommentParsed {
    pub fn new(comment_text: Box<String>, spannables: Box<Vec<Spannable>>) -> PostCommentParsed {
      PostCommentParsed {
        comment_text,
        spannables: spannables
      }
    }
  }

  enum ParsingRule {
    CustomRule(Box<dyn RuleHandler>)
  }

  pub struct CommentParser {
    rules: HashMap<String, ParsingRule>
  }

  impl CommentParser {

    pub fn new() -> CommentParser {
      return CommentParser {
        rules: HashMap::new()
      }
    }

    pub fn add_default_rules(&mut self) {
      self.rules.insert(String::from("a"), ParsingRule::CustomRule(Box::new(AnchorRuleHandler::new())));
      self.rules.insert(String::from("br"), ParsingRule::CustomRule(Box::new(LineBreakRuleHandler::new())));
    }

    pub fn process_element(&self, element: &Element, out_text_parts: &mut Vec<String>, out_spannables: &mut Vec<Spannable>) -> bool {
      let element_name = element.name.as_str();
      let rule_maybe = self.rules.get(element_name);

      let rule = match rule_maybe {
        None => panic!("No rule found for element with name \'{}\'", element_name),
        Some(_) => rule_maybe.unwrap()
      };

      return match rule {
        ParsingRule::CustomRule(rule_handler) => {
          rule_handler.handle(element, out_text_parts, out_spannables)
        }
      }
    }

  }
}