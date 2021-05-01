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
  let post_comment_raw = "Visual Novel General #4179<br><br>This general is for the discussion of English-translated Japanese visual novels.<br>\
All posting of untranslated visual novels belongs on <a href=\"//boards.4channel.org/jp/\" class=\"quotelink\">&gt;&gt;&gt;/jp/</a><br>E-celeb shitposting is not allowed.<br>\
Kindly use spoiler tags appropriately when discussing plot spoilers to facilitate smooth discussion.<br><br><span class=\"quote\">&gt;Having trouble with your VN? \
Try the following before you ask for tech support:</span><br>1. Be in Japanese locale<br>2. Read the Readme<br>3. Read the wiki below<br>4. \
Copy error messages with CTRL+C and paste them with CTRL+V into DeepL<br>5. Google it<br><br><span class=\"quote\">&gt;FAQs, Recommendations, \
and Other Useful Things:</span><br>http://visual-novels-general.wikia.<wbr>com/wiki/<br>https://sites.google.com/view/moech<wbr>art/<br>\
https://files.catbox.moe/143by7.png<wbr><br>https://i.imgur.com/3CDmFQm.jpg<br><br><span class=\"quote\">&gt;Need a novel with a specific element?</span><br>\
http://vndb.org/g<br><br><span class=\"quote\">&gt;Download Links:</span><br>https://pastebin.com/YTGdpqZL<br><br>\
Previous thread: <a href=\"/vg/thread/333581281#p333581281\" class=\"quotelink\">&gt;&gt;333581281</a>";

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
    HashSet::new(),
    HashSet::new()
  );

  let post_parser = PostParser::new(&post_parser_context);

  for post_raw in thread_raw.posts {
    if post_raw.com.is_none() {
      continue;
    }

    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed.unwrap();
    println!("comment: \n{}", post_comment_parsed.comment_text);

    for spannable in post_comment_parsed.spannables.iter() {
      println!("spannable: \n{}", spannable);
    }
  }
}