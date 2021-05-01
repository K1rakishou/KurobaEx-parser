#[cfg(test)]
mod test_main {
  use crate::PostRaw;
  use crate::post_parser::post_parser::{PostParser, PostParserContext};
  use crate::comment_parser::comment_parser::Spannable;
  use crate::comment_parser::comment_parser::SpannableData;
  use crate::comment_parser::comment_parser::PostLink;
  use crate::set;
  use std::collections::HashSet;

  fn run_test(
    post_id: u64,
    post_parser_context: &PostParserContext,
    raw_comment: &str,
    expected_parsed_comment: &str,
    expected_spannables: &Vec<Spannable>
  ) {
    let post_raw = PostRaw {
      post_id,
      com: Option::Some(String::from(raw_comment))
    };

    let post_parser = PostParser::new(&post_parser_context);
    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed.unwrap();
    let spannables = post_comment_parsed.spannables;

    assert_eq!(expected_parsed_comment, post_comment_parsed.comment_text.as_str());
    assert_eq!(expected_spannables.len(), spannables.len());

    for index in 0 .. spannables.len() {
      assert_eq!(expected_spannables[index], spannables[index]);
    }
  }

  #[test]
  fn post_parser_test_dead_quotes() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;";
    let expected_parsed_comment = ">>333520145 (DEAD)
>>333520391 (DEAD)
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"";

    let expected_spannables = vec![
      Spannable { start: 0, len: 18, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 333520145 }) },
      Spannable { start: 19, len: 18, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 333520391 }) }
    ];

    let post_parser_context = PostParserContext::new(
      1234567890u64,
      set!(),
      set!()
    );

    run_test(123456780u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_one_not_dead_quote() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;";
    let expected_parsed_comment = ">>333520145
>>333520391 (DEAD)
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 12, len: 18, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 333520391 }) }
    ];

    let post_parser_context = PostParserContext::new(
      1234567890u64,
      set!(),
      set!(333520145u64)
    );

    run_test(123456780u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_one_not_dead_quote_original_post() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;";
    let expected_parsed_comment = ">>333520145 (OP)
>>333520391 (DEAD)
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"";

    let expected_spannables = vec![
      Spannable { start: 0, len: 16, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 17, len: 18, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 333520391 }) }
    ];

    let post_parser_context = PostParserContext::new(
      333520145u64,
      set!(),
      set!(333520145u64)
    );

    run_test(123456780u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_one_not_dead_quote_original_post_you() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;";
    let expected_parsed_comment = ">>333520145 (OP) (You)
>>333520391 (DEAD)
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"";

    let expected_spannables = vec![
      Spannable { start: 0, len: 22, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 23, len: 18, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 333520391 }) }
    ];

    let post_parser_context = PostParserContext::new(
      333520145u64,
      set!(333520145u64),
      set!(333520145u64)
    );

    run_test(123456780u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_one_not_dead_quote_original_post_me() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;";
    let expected_parsed_comment = ">>333520145 (OP) (Me)
>>333520391 (DEAD)
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"";

    let expected_spannables = vec![
      Spannable { start: 0, len: 21, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 22, len: 18, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 333520391 }) }
    ];

    let post_parser_context = PostParserContext::new(
      333520145u64,
      set!(333520145u64),
      set!(333520145u64)
    );

    run_test(333520145u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_one_not_dead_quote_original_post_you_second_you() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;";
    let expected_parsed_comment = ">>333520145 (OP) (You)
>>333520391 (You)
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"";

    let expected_spannables = vec![
      Spannable { start: 0, len: 22, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 23, len: 17, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520391 }) }
    ];

    let post_parser_context = PostParserContext::new(
      333520145u64,
      set!(333520145u64, 333520391u64),
      set!(333520145u64, 333520391u64)
    );

    run_test(123u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_2() {
//     let post_comment_raw = "Visual Novel General #4179<br><br>This general is for the discussion of English-translated Japanese visual novels.<br>\
// All posting of untranslated visual novels belongs on <a href=\"//boards.4channel.org/jp/\" class=\"quotelink\">&gt;&gt;&gt;/jp/</a><br>E-celeb shitposting is not allowed.<br>\
// Kindly use spoiler tags appropriately when discussing plot spoilers to facilitate smooth discussion.<br><br><span class=\"quote\">&gt;Having trouble with your VN? \
// Try the following before you ask for tech support:</span><br>1. Be in Japanese locale<br>2. Read the Readme<br>3. Read the wiki below<br>4. \
// Copy error messages with CTRL+C and paste them with CTRL+V into DeepL<br>5. Google it<br><br><span class=\"quote\">&gt;FAQs, Recommendations, \
// and Other Useful Things:</span><br>http://visual-novels-general.wikia.<wbr>com/wiki/<br>https://sites.google.com/view/moech<wbr>art/<br>\
// https://files.catbox.moe/143by7.png<wbr><br>https://i.imgur.com/3CDmFQm.jpg<br><br><span class=\"quote\">&gt;Need a novel with a specific element?</span><br>\
// http://vndb.org/g<br><br><span class=\"quote\">&gt;Download Links:</span><br>https://pastebin.com/YTGdpqZL<br><br>\
// Previous thread: <a href=\"/vg/thread/333581281#p333581281\" class=\"quotelink\">&gt;&gt;333581281</a>";
//
//
//     run_test(post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  // <a href=\"/vg/thread/333790974#p333790974\" class=\"quotelink\">&gt;&gt;333790974</a><br><a href=\"/vg/thread/333790974#p333790974\" class=\"quotelink\">&gt;&gt;333790974</a><br><a href=\"/vg/thread/333790974#p333790974\" class=\"quotelink\">&gt;&gt;333790974</a>

}