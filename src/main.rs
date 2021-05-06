#![deny(warnings)]

use std::collections::HashSet;
use new_post_parser_lib::{ThreadRaw, PostRaw, set_mut, PostParserContext, PostParser};

fn main() {
  let post_comment_raw = "<span class=\"quote\">&gt;Read the sticky: <a href=\"/g/thread/76759434#p76759434\" \
    class=\"quotelink\">&gt;&gt;76759434</a></span><br><br><span class=\"quote\">&gt;GNU/Linux questions </span>\
    <a href=\"//boards.4channel.org/g/catalog#s=fglt\" class=\"quotelink\">&gt;&gt;&gt;/g/fglt</a><br><br>\
    <span class=\"quote\">&gt;Windows questions </span><a href=\"//boards.4channel.org/g/catalog#s=fwt\" class=\"quotelink\">&gt;&gt;&gt;/g/fwt</a>\
    <br><br><span class=\"quote\">&gt;PC building? </span><a href=\"//boards.4channel.org/g/catalog#s=pcbg\" class=\"quotelink\">&gt;&gt;&gt;/g/pcbg</a>\
    <br><br><span class=\"quote\">&gt;Programming questions </span><a href=\"//boards.4channel.org/g/catalog#s=dpt\" class=\"quotelink\">&gt;&gt;&gt;/g/dpt</a>\
    <br><br><span class=\"quote\">&gt;Good, cheap, laptops </span><a href=\"//boards.4channel.org/g/catalog#s=tpg\" class=\"quotelink\">&gt;&gt;&gt;/g/tpg</a><br>\
    <br><span class=\"quote\">&gt;Cheap electronics </span><a href=\"//boards.4channel.org/g/catalog#s=csg\" class=\"quotelink\">&gt;&gt;&gt;/g/csg</a><br>\
    <br><span class=\"quote\">&gt;Server questions </span><a href=\"//boards.4channel.org/g/catalog#s=hsg\" class=\"quotelink\">&gt;&gt;&gt;/g/hsg</a><br>\
    <br><span class=\"quote\">&gt;Buying headphones </span><a href=\"//boards.4channel.org/g/catalog#s=hpg\" class=\"quotelink\">&gt;&gt;&gt;/g/hpg</a><br>\
    <br>How to find/activate any version of Windows?<br>https://rentry.org/installwindows<br><br>Previous Thread <a href=\"/g/thread/81404563#p81404563\" class=\"quotelink\">&gt;&gt;81404563</a>";

  let thread_raw = ThreadRaw {
    posts: vec![
      PostRaw {
        post_id: 333696415u64,
        post_sub_id: 0u64,
        com: Option::Some(String::from(post_comment_raw))
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
    if post_raw.com.is_none() {
      continue;
    }

    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed.unwrap();
    println!("comment: \n{}", post_comment_parsed.parsed_text);

    for spannable in post_comment_parsed.spannables.iter() {
      println!("{}", spannable);
    }
  }
}