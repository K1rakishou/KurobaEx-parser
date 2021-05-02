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
  fn post_parser_test_greentext_inside_spoiler() {
    let post_comment_raw = "<a href=\"#p333890765\" class=\"quotelink\">&gt;&gt;333890765</a><br><span class=\"quote\">&gt;letting \"realism\" \
stop you</span><br><s>Should I use a female version of my name for maximal self-insertion</s>?";
    let expected_parsed_comment = ">>333890765\n>letting \"realism\" stop you\nShould I use a female version of my name for maximal self-insertion?";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333890765 }) },
      Spannable { start: 11, len: 28, spannable_data: SpannableData::GreenText },
      Spannable { start: 39, len: 68, spannable_data: SpannableData::Spoiler },
    ];

    let post_parser_context = PostParserContext::new(
      333859392u64,
      set!(),
      set!(333890765u64)
    );

    run_test(333890765u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_quote_inside_spoiler_inside_greentext() {
    let post_comment_raw = "<span class=\"quote\"><s><a href=\"#p333863078\" class=\"quotelink\">&gt;&gt;333863078</a><wbr></s></span>";
    let expected_parsed_comment = ">>333863078";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333863078 }) },
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Spoiler },
      Spannable { start: 0, len: 11, spannable_data: SpannableData::GreenText },
    ];

    let post_parser_context = PostParserContext::new(
      1234u64,
      set!(),
      set!(333863078u64)
    );

    run_test(1235u64, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  // Data for dead link handler test
  // <a href="#p333918351" class="quotelink">&gt;&gt;333918351</a><br>Because JOPs can just go to their dedicated thread on /jp/. &gt;<span class="deadlink">&gt;&gt;34511118</span>

}