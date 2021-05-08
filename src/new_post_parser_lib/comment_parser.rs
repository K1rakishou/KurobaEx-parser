pub mod comment_parser {
  use std::collections::{HashMap, HashSet};

  use crate::rules::anchor::AnchorRuleHandler;
  use crate::rules::span::SpanHandler;
  use crate::rules::rule_handler::RuleHandler;
  use crate::rules::line_break::LineBreakRuleHandler;
  use std::fmt;
  use crate::{set_immut, set_mut, TextPart};
  use crate::{PostRaw, PostParserContext, Element, ParsingRule, CommentParser, PostLink, SpannableData, Spannable, ParsedSpannableText};
  use crate::rules::spoiler::SpoilerHandler;
  use crate::rules::table_row::TableRowHandler;
  use crate::rules::bold::BoldHandler;
  use crate::rules::abbr::AbbrHandler;
  use std::rc::Rc;
  use crate::rules::style::StyleHandler;
  use linked_hash_map::LinkedHashMap;

  const TAG: &str = "CommentParser";

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

  impl fmt::Display for SpannableData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      return match self {
        SpannableData::Link(post_link) => {
          write!(f, "PostLink(post_link: {})", post_link)
        }
        SpannableData::Spoiler => {
          write!(f, "Spoiler()")
        }
        SpannableData::GreenText => {
          write!(f, "GreenText()")
        }
        SpannableData::BoldText => {
          write!(f, "BoldText()")
        }
        SpannableData::TextForegroundColorRaw { color_hex: raw_color } => {
          write!(f, "TextForegroundColorRaw(raw_color: {})", raw_color)
        }
        SpannableData::TextBackgroundColorRaw { color_hex: raw_color } => {
          write!(f, "TextBackgroundColorRaw(raw_color: {})", raw_color)
        }
        SpannableData::TextForegroundColorId { color_id } => {
          write!(f, "TextForegroundColorId(color_id: {})", color_id)
        }
        SpannableData::TextBackgroundColorId { color_id } => {
          write!(f, "TextBackgroundColorId(color_id: {})", color_id)
        }
        SpannableData::FontSize { size } => {
          write!(f, "FontSize(size: {})", size)
        }
        SpannableData::FontWeight { weight } => {
          write!(f, "FontWeight(weight: {})", weight)
        }
      }
    }
  }

  impl Spannable {
    pub fn is_valid(&self) -> bool {
      return self.len > 0
    }
  }

  impl fmt::Display for Spannable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "Spannable(start: {}, len: {}, spannable_data: {})", self.start, self.len, self.spannable_data)
    }
  }

  impl fmt::Display for ParsedSpannableText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ParsedSpannableText(comment_text: {}", self.parsed_text).unwrap();

      for spannable in self.spannables.iter() {
        write!(f, ", spannable: {}", spannable).unwrap();
      }

      return write!(f, ")");
    }
  }

  impl ParsedSpannableText {
    pub fn new(comment_raw: &str, comment_text: Box<String>, spannables: Box<Vec<Spannable>>) -> ParsedSpannableText {
      ParsedSpannableText {
        original_text: String::from(comment_raw),
        parsed_text: comment_text,
        spannables
      }
    }

    pub fn empty() -> ParsedSpannableText {
      return ParsedSpannableText {
        original_text: String::new(),
        parsed_text: Box::new(String::new()),
        spannables: Box::new(Vec::new())
      }
    }
  }

  impl ParsingRule {
    pub fn new(tag: &str, req_attributes: HashSet<String>, handler: Rc<dyn RuleHandler>) -> ParsingRule {
      ParsingRule {
        tag: String::from(tag),
        req_attributes,
        handler
      }
    }

    pub fn high_priority(&self) -> bool {
      return self.req_attributes.len() > 0;
    }

    pub fn applies(&self, element: &Element) -> bool {
      if self.req_attributes.is_empty() {
        return true
      }

      for req_attribute in &self.req_attributes {
        if element.get_attr_value(req_attribute).is_some() {
          return true;
        }

        if element.has_class(&req_attribute) {
          return true;
        }
      }

      return false;
    }

  }

  impl CommentParser<'_> {

    pub fn new(post_parser_context: &PostParserContext) -> CommentParser {
      return CommentParser {
        post_parser_context,
        matching_rules: LinkedHashMap::new(),
        replacement_rules: HashMap::new()
      }
    }

    fn add_matching_rule(&mut self, rule: Rc<ParsingRule>) {
      if !self.matching_rules.contains_key(&rule.tag) {
        self.matching_rules.insert(String::from(&rule.tag), Vec::new());
      }

      self.matching_rules.get_mut(&rule.tag).unwrap().push(rule);
    }

    pub fn add_replacement_rule(&mut self, pattern: &str, value: &str) {
      let result = self.replacement_rules.insert(String::from(pattern), String::from(value));

      if result.is_some() {
        panic!("add_replacement() pattern {} with value {} was already added! Old value: {}", pattern, value, result.unwrap());
      }
    }

    pub fn add_default_matching_rules(&mut self) {
      // Wildcard rules go first
      self.add_matching_rule(Rc::new(ParsingRule::new("*", set_mut!("style".to_string()), Rc::new(StyleHandler::new()))));

      // Then go rules that require specific attributes
      self.add_matching_rule(Rc::new(ParsingRule::new("span", set_mut!("abbr".to_string()), Rc::new(AbbrHandler::new()))));

      // Then go general rules for the whole tag
      self.add_matching_rule(Rc::new(ParsingRule::new("span", set_immut!(), Rc::new(SpanHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("a", set_immut!(), Rc::new(AnchorRuleHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("br", set_immut!(), Rc::new(LineBreakRuleHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("s", set_immut!(), Rc::new(SpoilerHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("tr", set_immut!(), Rc::new(TableRowHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("b", set_immut!(), Rc::new(BoldHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("strong", set_immut!(), Rc::new(BoldHandler::new()))));
    }

    pub fn get_matching_rules(&self, element_name: &str) -> Option<Vec<Rc<ParsingRule>>> {
      let mut all_rules: Vec<Rc<ParsingRule>> = Vec::with_capacity(16);

      for (_, rules) in &self.matching_rules {
        for rule in rules {
          if rule.tag == "*" || rule.tag == element_name {
            all_rules.push(rule.clone());
          }
        }
      }

      return Option::Some(all_rules.to_vec());
    }

    /// returns true if we managed to parse this node fully and don't need to go deeper for child nodes.
    pub fn pre_process_element(
      &self,
      post_raw: &PostRaw,
      element: &Element,
      out_text_parts: &mut Vec<TextPart>,
      out_spannables: &mut Vec<Spannable>
    ) -> bool {
      let element_name = element.name.as_str();
      let rules_maybe = self.get_matching_rules(element_name);

      let rules = match rules_maybe {
        None => return false,
        Some(_) => rules_maybe.unwrap()
      };

      for index in 0..2 {
        let high_priority = index == 0;

        for rule in &rules {
          if rule.high_priority() == high_priority && rule.applies(element) {
            if rule.handler.pre_handle(post_raw, self.post_parser_context, element, out_text_parts, out_spannables) {
              return true
            }
          }
        }
      }

      return false;
    }

    /// Called after element's child nodes were all processed. Useful when you need to know the len of
    /// child nodes text
    pub fn post_process_element(
      &self,
      post_raw: &PostRaw,
      element: &Element,
      prev_out_text_parts_index: usize,
      out_text_parts: &mut Vec<TextPart>,
      prev_out_spannables_index: usize,
      out_spannables: &mut Vec<Spannable>
    ) {
      let element_name = element.name.as_str();
      let rules_maybe = self.get_matching_rules(element_name);

      let rules = match rules_maybe {
        None => return,
        Some(_) => rules_maybe.unwrap()
      };

      for index in 0..2 {
        let high_priority = index == 0;

        for rule in &rules {
          if rule.high_priority() == high_priority && rule.applies(element) {
            rule.handler.post_handle(
              post_raw,
              self.post_parser_context,
              element,
              prev_out_text_parts_index,
              out_text_parts,
              prev_out_spannables_index,
              out_spannables
            )
          }
        }
      }
    }
  }
}