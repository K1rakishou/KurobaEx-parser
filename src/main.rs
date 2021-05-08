#![deny(warnings)]

use std::collections::HashSet;
use new_post_parser_lib::{ThreadRaw, PostRaw, set_mut, PostParserContext, PostParser};

fn main() {
  let post_comment_raw = "<span style=\"font-weight:600;font-size:150%;line-height:1.5;\">Begging or asking for &#039;free \
  money&#039;/crypto is strictly forbidden. Encouraging beggars, or posting any kind of &#039;free money&#039; \
  offer is also strictly forbidden.</span>";

  let thread_raw = ThreadRaw {
    posts: vec![
      PostRaw {
        post_id: 333696415u64,
        post_sub_id: 0u64,
        com: String::from(post_comment_raw)
      }
    ]
  };

  let post_parser_context = PostParserContext::new(
    "4chan",
    "g",
    333696415u64,
    set_mut!(),
    set_mut!(333918351)
  );

  let post_parser = PostParser::new(&post_parser_context);

  for post_raw in thread_raw.posts {
    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed;
    println!("comment: \n{}", post_comment_parsed.parsed_text);

    for spannable in post_comment_parsed.spannables.iter() {
      println!("{}", spannable);
    }
  }
}