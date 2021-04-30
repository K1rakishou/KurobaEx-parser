use parser::parser::*;

#[macro_use]
extern crate lazy_static;

mod parser;
mod comment_parser;

mod rules {
  pub mod anchor;
  pub mod line_break;
  pub mod rule_handler;
  pub mod span;
}

mod tests {
  pub mod post_parser_tests;
  pub mod html_parser_tests;
}

mod html_parser {
  pub mod node;
  pub mod element;
  pub mod parser;
}

pub struct PostRaw {
  com: Option<String>,
}

struct ThreadRaw {
  posts: Vec<PostRaw>,
}

fn main() {
  let post_comment_raw = "<a href=\"#p333650561\" class=\"quotelink\">&gt;&gt;333650561</a><br><span class=\"quote\">&gt;what&#039;s the best alternative</span><br>Reps";
  let thread_raw = ThreadRaw {
      posts: vec![
          PostRaw {
              com: Option::Some(String::from(post_comment_raw))
          }
      ]
  };

  let post_parser = PostParser::new();

  for post_raw in thread_raw.posts {
      if post_raw.com.is_none() {
          continue;
      }

      let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed.unwrap();
      println!("comment: \n{}", post_comment_parsed.comment_text);

      for spannable in post_comment_parsed.spannables.iter() {
          println!("spannable: \n{:?}", spannable);
      }
  }
}