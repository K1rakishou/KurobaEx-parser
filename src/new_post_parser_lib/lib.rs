#[macro_use]
extern crate lazy_static;

use std::collections::{HashSet, HashMap};
use linked_hash_map::LinkedHashMap;
use crate::html_parser::node::Node;
use crate::rules::rule_handler::RuleHandler;

mod post_parser;
mod comment_parser;
mod parsing_error;

mod rules {
  pub mod anchor;
  pub mod line_break;
  pub mod word_break;
  pub mod rule_handler;
  pub mod span;
  pub mod spoiler;
}

mod html_parser {
  pub mod node;
  pub mod element;
  pub mod parser;
}

pub mod util {
  pub mod macroses;
  pub mod helpers;
}

pub struct PostRaw {
  pub post_id: u64,
  pub com: Option<String>,
}

pub struct ThreadRaw {
  pub posts: Vec<PostRaw>,
}

#[derive(Debug)]
pub struct PostParserContext {
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
  rules: HashMap<String, Vec<Box<ParsingRule>>>
}

pub struct ParsingRule {
  tag: String,
  req_classes: HashSet<String>,
  handler: Box<dyn RuleHandler>
}

pub struct ParsedPost {
  pub post_comment_parsed: Option<PostCommentParsed>,
}

pub struct PostCommentParsed {
  pub comment_text: Box<String>,
  pub spannables: Box<Vec<Spannable>>
}

#[derive(Debug, PartialEq)]
pub struct Spannable {
  pub start: usize,
  pub len: usize,
  pub spannable_data: SpannableData
}

#[derive(Debug, PartialEq)]
pub enum SpannableData {
  Link(PostLink),
  Spoiler,
  GreenText,
}

#[derive(Debug, PartialEq)]
pub enum PostLink {
  Quote { post_no: u64 },
  Dead { post_no: u64 },
  UrlLink { link: String },
  BoardLink { board_code: String },
  SearchLink { board_code: String, search_query: String },
  ThreadLink { board_code: String, thread_no: u64, post_no: u64 }
}

pub struct HtmlParser {}