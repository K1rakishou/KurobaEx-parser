#![deny(warnings)]

use std::collections::HashSet;
use new_post_parser_lib::{ThreadRaw, PostRaw, set_mut, PostParserContext, PostParser};

fn main() {
  let post_comment_raw = "<span class=\"quote\">&gt;Cinco de Mayo (pronounced [ˈsiŋko̞ ðe̞ ˈma̠ʝo̞] in Mexico, Spanish for &quot;Fifth of May&quot;) \
    is an annual celebration held on May 5. The date is observed to commemorate the Mexican Army&#039;s victory over the French Empire at the Battle of Puebla, \
    on May 5, 1862, under the leadership of General Ignacio Zaragoza.[1][2] The victory of the smaller Mexican force against a larger French force was \
    a boost to morale for the Mexicans. Zaragoza died months after the battle due to illness. A year after the battle, a larger French force defeated the \
    Mexican army at the Second Battle of Puebla, and Mexico City soon fell to the invaders.</span><br>what?";

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