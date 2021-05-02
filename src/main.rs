use post_parser::post_parser::*;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

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

mod util {
  pub mod macroses;
  pub mod helpers;
}

mod tests {
  pub mod post_parser_tests;
  pub mod html_parser_tests;
}

pub struct PostRaw {
  post_id: u64,
  com: Option<String>,
}

struct ThreadRaw {
  posts: Vec<PostRaw>,
}

fn main() {
  let post_comment_raw = "https://www.youtube.com/watch?v=57tu8AtKf9E
https://boards.4channel.org/vg/thread/333979978 test https://boards.4channel.org/v/

http://visual-novels-general.wikia.com/wiki/
https://sites.google.com/view/moechart/
https://files.catbox.moe/143by7.png (embed)
https://i.imgur.com/3CDmFQm.jpg (embed)
http://vndb.org/g
https://pastebin.com/YTGdpqZL (embed)
https://pastebin.com/YTGdpqZL
http://2.com
https://pastebin.com/1 https://w4534gerhnrh.com/2 https://pastebin.com/3

https://www.youtube.com/watch?v=57tu8AtKf9E";

  let thread_raw = ThreadRaw {
    posts: vec![
      PostRaw {
        post_id: 333696415u64,
        com: Option::Some(String::from(post_comment_raw))
      }
    ]
  };

  let post_parser_context = PostParserContext::new(
    333696415u64,
    set!(),
    set!(333918351)
  );

  let post_parser = PostParser::new(&post_parser_context);

  for post_raw in thread_raw.posts {
    if post_raw.com.is_none() {
      continue;
    }

    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed.unwrap();
    println!("comment: \n{}", post_comment_parsed.comment_text);

    for spannable in post_comment_parsed.spannables.iter() {
      println!("{}", spannable);
    }
  }
}