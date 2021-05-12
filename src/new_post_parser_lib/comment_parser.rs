pub mod comment_parser {
  use std::collections::{HashMap, HashSet};

  use crate::rules::anchor::AnchorRuleHandler;
  use crate::rules::span::SpanHandler;
  use crate::rules::rule_handler::RuleHandler;
  use crate::rules::line_break::LineBreakRuleHandler;
  use std::fmt;
  use crate::{empty_set, set_of, TextPart, Attribute};
  use crate::{PostRaw, PostParserContext, Element, ParsingRule, CommentParser, PostLink, SpannableData, Spannable, ParsedSpannableText};
  use crate::rules::spoiler::SpoilerHandler;
  use crate::rules::table_row::TableRowHandler;
  use crate::rules::bold::BoldHandler;
  use crate::rules::abbr::AbbrHandler;
  use std::rc::Rc;
  use crate::rules::style::StyleHandler;
  use linked_hash_map::LinkedHashMap;
  use std::fmt::Debug;
  use crate::rules::pre::PreHandler;
  use crate::rules::table_data::TableDataHandler;
  use crate::rules::table::TableHandler;

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
        SpannableData::Monospace => {
          write!(f, "Monospace()")
        }
        SpannableData::ThemeJson { theme_name, is_light_theme } => {
          write!(f, "ThemeJson(theme_name: {}, is_light_theme: {})", theme_name, is_light_theme)
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

  impl Debug for ParsingRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "ParsingRule(tag: {}, req_attributes: {:?})", self.tag_name, self.required_attributes)
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

  impl Attribute {
    fn with_name(attr_name: &str) -> Attribute {
      return Attribute {
        attr_name: attr_name.to_string(),
        attr_value: Option::None
      }
    }

    fn with_name_and_value(attr_name: &str, attr_value: &str) -> Attribute {
      return Attribute {
        attr_name: attr_name.to_string(),
        attr_value: Option::Some(attr_value.to_string())
      }
    }

    fn with_class(attr_value: &str) -> Attribute {
      return Attribute {
        attr_name: "class".to_string(),
        attr_value: Option::Some(attr_value.to_string())
      }
    }
  }

  impl ParsingRule {
    pub fn new(tag: &str, req_attributes: HashSet<Attribute>, handler: Rc<dyn RuleHandler>) -> ParsingRule {
      ParsingRule {
        tag_name: String::from(tag),
        required_attributes: req_attributes,
        handler
      }
    }

    pub fn high_priority(&self) -> bool {
      return self.required_attributes.len() > 0;
    }

    pub fn applies(&self, element: &Element) -> bool {
      if self.required_attributes.is_empty() {
        return true
      }

      for req_attribute in &self.required_attributes {
        if req_attribute.attr_name == "*" {
          continue;
        }

        let attr_name = &req_attribute.attr_name;
        let attr_value = element.attributes.get(attr_name);

        if req_attribute.attr_value.is_none() {
          continue;
        }

        let req_attr_value = req_attribute.attr_value.as_ref().unwrap();
        if attr_value.is_none() {
          return false;
        }

        let attr_value = attr_value.unwrap();
        if req_attr_value != attr_value {
          return false;
        }
      }

      return true;
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
      if !self.matching_rules.contains_key(&rule.tag_name) {
        self.matching_rules.insert(String::from(&rule.tag_name), Vec::new());
      }

      self.matching_rules.get_mut(&rule.tag_name).unwrap().push(rule);
    }

    pub fn add_replacement_rule(&mut self, pattern: &str, value: &str) {
      let result = self.replacement_rules.insert(String::from(pattern), String::from(value));

      if result.is_some() {
        panic!("add_replacement() pattern {} with value {} was already added! Old value: {}", pattern, value, result.unwrap());
      }
    }

    pub fn add_default_matching_rules(&mut self) {
      // Wildcard rules go first
      self.add_matching_rule(Rc::new(ParsingRule::new("*", set_of!(Attribute::with_name("style")), Rc::new(StyleHandler::new()))));

      // Then go rules that require specific attributes
      self.add_matching_rule(Rc::new(ParsingRule::new("span", set_of!(Attribute::with_class("abbr")), Rc::new(AbbrHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("pre", set_of!(Attribute::with_name_and_value("*", "prettyprint")), Rc::new(PreHandler::new()))));

      // Then go general rules for the whole tag
      self.add_matching_rule(Rc::new(ParsingRule::new("span", empty_set!(), Rc::new(SpanHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("a", empty_set!(), Rc::new(AnchorRuleHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("br", empty_set!(), Rc::new(LineBreakRuleHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("s", empty_set!(), Rc::new(SpoilerHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("tr", empty_set!(), Rc::new(TableRowHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("td", empty_set!(), Rc::new(TableDataHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("b", empty_set!(), Rc::new(BoldHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("strong", empty_set!(), Rc::new(BoldHandler::new()))));
      self.add_matching_rule(Rc::new(ParsingRule::new("table", empty_set!(), Rc::new(TableHandler::new()))));
    }

    pub fn get_matching_rules(&self, element: &Element) -> Option<Vec<Rc<ParsingRule>>> {
      let mut all_rules: Vec<Rc<ParsingRule>> = Vec::with_capacity(16);

      for (_, rules) in &self.matching_rules {
        for rule in rules {
          if rule.tag_name != "*" && rule.tag_name != element.tag_name {
            continue;
          }

          let mut all_req_attributes_match = true;

          for required_attribute in &rule.required_attributes {
            if &required_attribute.attr_name != "*" {
              let element_attr_value_maybe = element.attributes.get(&required_attribute.attr_name);

              let element_attr_value = if let Option::None = element_attr_value_maybe {
                all_req_attributes_match = false;
                break;
              } else {
                element_attr_value_maybe.unwrap()
              };

              let required_attr_value = if let Option::None = required_attribute.attr_value {
                continue;
              } else {
                required_attribute.attr_value.as_ref().unwrap()
              };

              if element_attr_value != required_attr_value {
                all_req_attributes_match = false;
                break;
              }
            }
          }

          if !all_req_attributes_match {
            continue;
          }

          all_rules.push(rule.clone());
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
      let rules_maybe = self.get_matching_rules(element);

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
      let rules_maybe = self.get_matching_rules(element);

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