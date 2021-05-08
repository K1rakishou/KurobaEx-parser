#![deny(warnings)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::collections::{HashSet, HashMap};
use linked_hash_map::LinkedHashMap;
use crate::html_parser::node::Node;
use crate::rules::rule_handler::RuleHandler;
use core::fmt;
use std::rc::Rc;

mod post_parser;
mod comment_parser;
mod parsing_error;

mod rules {
  pub mod anchor;
  pub mod line_break;
  pub mod rule_handler;
  pub mod span;
  pub mod spoiler;
  pub mod table_row;
  pub mod bold;
  pub mod abbr;
  pub mod style;
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
}

#[derive(Debug)]
pub struct TextPart {
  text: String,
  characters_count: usize
}

impl TextPart {
  fn new(text: String) -> TextPart {
    let characters_count = text.chars().count();

    return TextPart {
      text,
      characters_count
    };
  }
}

#[derive(Debug)]
pub struct PostRaw {
  pub post_id: u64,
  pub post_sub_id: u64,
  pub com: String,
}

#[derive(Debug)]
pub struct ThreadRaw {
  pub posts: Vec<PostRaw>,
}

#[derive(Debug)]
pub struct PostParserContext {
  site_name: String,
  board_code: String,
  thread_id: u64,
  my_replies: HashSet<u64>,
  thread_posts: HashSet<u64>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
  pub name: String,
  pub attributes: LinkedHashMap<String, String>,
  pub children: Vec<Node>,
  pub is_void_element: bool,
}

pub struct PostParser<'a> {
  post_parser_context: &'a PostParserContext,
  comment_parser: Box<CommentParser<'a>>,
}

pub struct CommentParser<'a> {
  post_parser_context: &'a PostParserContext,
  matching_rules: LinkedHashMap<String, Vec<Rc<ParsingRule>>>,
  /// [Key] what pattern in the comment text needs to be replaced with [Value]
  replacement_rules: HashMap<String, String>
}

pub struct ParsingRule {
  tag: String,
  req_attributes: HashSet<String>,
  handler: Rc<dyn RuleHandler>
}

pub struct ParsedPost {
  pub site_name: String,
  pub board_code: String,
  pub thread_id: u64,
  pub post_id: u64,
  pub post_sub_id: u64,
  pub post_comment_parsed: ParsedSpannableText,
}

pub struct ParsedSpannableText {
  pub original_text: String,
  pub parsed_text: Box<String>,
  pub spannables: Box<Vec<Spannable>>
}

#[derive(Debug, PartialEq)]
pub struct Spannable {
  pub start: usize,
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
  TextForegroundColorRaw { color_hex: String },
  TextBackgroundColorRaw { color_hex: String },
  TextForegroundColorId { color_id: ChanThemeColorId },
  TextBackgroundColorId { color_id: ChanThemeColorId }
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