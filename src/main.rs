#![deny(warnings)]

use std::collections::HashSet;
use new_post_parser_lib::{ThreadRaw, PostRaw, set_of, PostParserContext, PostParser};

fn main() {
  let post_comment_raw = "●/dng/ Battle City (June 27, 1600 UTC): https://challonge.com/dngbattlecity";

  let thread_raw = ThreadRaw {
    posts: vec![
      PostRaw::new("4chan", "g", 333696415u64, 333696415u64, 0u64, post_comment_raw)
    ]
  };

  let post_parser_context = PostParserContext::new(
    set_of!(),
    set_of!(333863078)
  );

  let post_parser = PostParser::new(&post_parser_context);

  for post_raw in thread_raw.posts {
    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed;
    println!("comment: \n{}", post_comment_parsed.parsed_text);
    assert_eq!("", *post_comment_parsed.parsed_text);

    for spannable in post_comment_parsed.spannables.iter() {
      println!("{}", spannable);
    }
  }
}