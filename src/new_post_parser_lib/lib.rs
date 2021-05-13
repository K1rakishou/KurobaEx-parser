#![deny(warnings)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::collections::{HashSet, HashMap};
use linked_hash_map::LinkedHashMap;
use crate::html_parser::node::Node;
use crate::rules::rule_handler::RuleHandler;
use core::{fmt};
use std::rc::Rc;
use std::fmt::{Debug};

mod post_parser;
mod comment_parser;
mod parsing_error;

mod rules {
  pub mod anchor;
  pub mod line_break;
  pub mod rule_handler;
  pub mod span;
  pub mod spoiler;
  pub mod table;
  pub mod table_row;
  pub mod table_data;
  pub mod bold;
  pub mod abbr;
  pub mod style;
  pub mod pre;
}

pub mod html_parser {
  pub mod node;
  pub mod element;
  pub mod parser;
}

pub mod util {
  pub mod macroses;
  pub mod helpers;
  pub mod color_decoder;
  pub mod style_tag_value_decoder;
  pub mod theme_json_extractor;
}

#[derive(Debug)]
pub struct TextPart {
  text: String,
  characters_count: usize,
  bytes_count: usize
}

impl TextPart {
  fn new(text: String) -> TextPart {
    let characters_count = text.chars().count();
    let bytes_count = text.len();

    return TextPart {
      text,
      characters_count,
      bytes_count
    };
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SiteDescriptor {
  pub site_name: String
}

#[derive(Debug, Eq, PartialEq)]
pub struct BoardDescriptor {
  pub site_descriptor: SiteDescriptor,
  pub board_code: String
}

#[derive(Debug, Eq, PartialEq)]
pub struct ThreadDescriptor {
  pub board_descriptor: BoardDescriptor,
  pub thread_no: u64
}

#[derive(Debug, Eq, PartialEq)]
pub struct PostDescriptor {
  pub thread_descriptor: ThreadDescriptor,
  pub post_no: u64,
  pub post_sub_no: u64,
}

#[derive(Debug)]
pub struct PostRaw {
  pub post_descriptor: PostDescriptor,
  pub com: String,
}

impl PostRaw {
  pub fn site_name(&self) -> &String {
    return &self.post_descriptor.thread_descriptor.board_descriptor.site_descriptor.site_name;
  }

  pub fn board_code(&self) -> &String {
    return &self.post_descriptor.thread_descriptor.board_descriptor.board_code;
  }

  pub fn thread_no(&self) -> u64 {
    return self.post_descriptor.thread_descriptor.thread_no;
  }

  pub fn post_no(&self) -> u64 {
    return self.post_descriptor.post_no;
  }

  pub fn post_sub_no(&self) -> u64 {
    return self.post_descriptor.post_sub_no;
  }

  pub fn is_quoting_original_post(&self, quote_post_id: u64) -> bool {
    return self.thread_no() == quote_post_id;
  }

  pub fn new(site_name: &str, board_code: &str, thread_no: u64, post_no: u64, post_sub_no: u64, raw_comment: &str) -> PostRaw {
    let thread_descriptor = ThreadDescriptor {
      board_descriptor: BoardDescriptor {
        site_descriptor: SiteDescriptor { site_name: site_name.to_string() },
        board_code: board_code.to_string()
      },
      thread_no
    };

    return PostRaw {
      post_descriptor: PostDescriptor {
        thread_descriptor,
        post_no,
        post_sub_no
      },
      com: String::from(raw_comment)
    };
  }
}

#[derive(Debug)]
pub struct ThreadRaw {
  pub posts: Vec<PostRaw>,
}

#[derive(Debug)]
pub struct PostParserContext {
  my_replies: HashSet<u64>,
  thread_posts: HashSet<u64>
}

#[derive(Clone, PartialEq)]
pub struct Element {
  pub tag_name: String,
  pub attributes: LinkedHashMap<String, String>,
  pub children: Vec<Node>,
  pub is_void_element: bool,
}

pub struct PostParser<'a> {
  post_parser_context: &'a PostParserContext,
  pub comment_parser: Box<CommentParser<'a>>,
}

pub struct CommentParser<'a> {
  post_parser_context: &'a PostParserContext,
  matching_rules: LinkedHashMap<String, Vec<Rc<ParsingRule>>>,
  /// [Key] what pattern in the comment text needs to be replaced with [Value]
  replacement_rules: HashMap<String, String>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
  attr_name: String,
  attr_value: Option<String>
}

pub struct ParsingRule {
  tag_name: String,
  required_attributes: HashSet<Attribute>,
  handler: Rc<dyn RuleHandler>
}

pub struct ParsedPost {
  pub site_name: String,
  pub board_code: String,
  pub thread_no: u64,
  pub post_no: u64,
  pub post_sub_no: u64,
  pub post_comment_parsed: ParsedSpannableText,
}

pub struct ParsedSpannableText {
  pub original_text: String,
  pub parsed_text: Box<String>,
  pub spannables: Box<Vec<Spannable>>
}

#[derive(Debug, PartialEq)]
pub struct Spannable {
  // unicode characters (not u8!)
  pub start: usize,
  // unicode characters (not u8!)
  pub len: usize,
  pub spannable_data: SpannableData
}

/// When changing this DO NOT FORGET to also change com.github.k1rakishou.core_themes.ChanThemeColorId !!!
#[derive(Debug, PartialEq, Clone)]
pub enum ChanThemeColorId {
  PostSubjectColor = 0,
  PostNameColor = 1,
  AccentColor = 2,
  PostInlineQuoteColor = 3,
  PostQuoteColor = 4,
  BackColorSecondary = 5,
  PostLinkColor = 6,
  TextColorPrimary = 7,
}

impl fmt::Display for ChanThemeColorId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return match self {
      ChanThemeColorId::PostSubjectColor => {
        write!(f, "ChanThemeColorId::PostSubjectColor")
      }
      ChanThemeColorId::PostNameColor => {
        write!(f, "ChanThemeColorId::PostNameColor")
      }
      ChanThemeColorId::AccentColor => {
        write!(f, "ChanThemeColorId::AccentColor")
      }
      ChanThemeColorId::PostInlineQuoteColor => {
        write!(f, "ChanThemeColorId::PostInlineQuoteColor")
      }
      ChanThemeColorId::PostQuoteColor => {
        write!(f, "ChanThemeColorId::PostQuoteColor")
      }
      ChanThemeColorId::BackColorSecondary => {
        write!(f, "ChanThemeColorId::BackColorSecondary")
      }
      ChanThemeColorId::PostLinkColor => {
        write!(f, "ChanThemeColorId::PostLinkColor")
      }
      ChanThemeColorId::TextColorPrimary => {
        write!(f, "ChanThemeColorId::TextColorPrimary")
      }
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SpannableData {
  Link(PostLink),
  Spoiler,
  GreenText,
  BoldText,
  // font-size:22px;font-size:150%;
  FontSize { size: String },
  // TODO: FontWeight is not implemented yet
  // font-weight:600;font-weight:bold
  FontWeight { weight: String },
  // color:#fd4d32
  Monospace,
  TextForegroundColorRaw { color_hex: String },
  TextBackgroundColorRaw { color_hex: String },
  TextForegroundColorId { color_id: ChanThemeColorId },
  TextBackgroundColorId { color_id: ChanThemeColorId },
  ThemeJson { theme_name: String, is_light_theme: bool }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostLink {
  Quote { post_no: u64 },
  Dead { post_no: u64 },
  UrlLink { link: String },
  BoardLink { board_code: String },
  SearchLink { board_code: String, search_query: String },
  ThreadLink { board_code: String, thread_no: u64, post_no: u64 }
}

pub struct HtmlParser {}