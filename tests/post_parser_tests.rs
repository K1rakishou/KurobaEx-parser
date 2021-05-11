#[cfg(test)]
mod test_main {
  use new_post_parser_lib::{PostParserContext, Spannable, PostRaw, PostParser, SpannableData, PostLink, set_of, Element};
  use std::collections::HashSet;

  fn create_post_parser_context(
    thread_id: u64,
    my_replies: HashSet<u64>,
    thread_posts: HashSet<u64>
  ) -> PostParserContext {
    PostParserContext::new(
      "4chan",
      "g",
      thread_id,
      my_replies,
      thread_posts
    )
  }

  fn run_test(
    post_id: u64,
    post_parser_context: &PostParserContext,
    raw_comment: &str,
    expected_parsed_comment: &str,
    expected_spannables: &Vec<Spannable>
  ) {
    let post_raw = PostRaw {
      post_id,
      post_sub_id: 0u64,
      com: String::from(raw_comment)
    };

    let post_parser = PostParser::new(&post_parser_context);
    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed;
    let spannables = post_comment_parsed.spannables;

    let parsed_text = post_comment_parsed.parsed_text.as_str();

    assert_eq!(expected_parsed_comment, parsed_text);
    assert_eq!(expected_spannables.len(), spannables.len());

    for index in 0 .. spannables.len() {
      let expected_spannable = &expected_spannables[index];
      let actual_spannable = &spannables[index];

      assert_eq!(expected_spannable, actual_spannable);

      assert!(actual_spannable.start >= 0, "start={}", actual_spannable.start);

      assert!(
        (actual_spannable.start + actual_spannable.len) <= parsed_text.bytes().len(),
        "end={}, chars_count={}",
        actual_spannable.start + actual_spannable.len,
        parsed_text.chars().count()
      );
    }
  }

  #[test]
  fn post_parser_test_empty_tag() {
    let post_comment_raw = "<b></b>";
    let expected_parsed_comment = "";

    let expected_spannables = vec![];

    let post_parser_context = create_post_parser_context(
      1234567890,
      set_of!(),
      set_of!()
    );

    run_test(123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_empty_tag_with_text() {
    let post_comment_raw = "Test1<b></b>Test2";
    let expected_parsed_comment = "Test1Test2";

    let expected_spannables = vec![];

    let post_parser_context = create_post_parser_context(
      1234567890,
      set_of!(),
      set_of!()
    );

    run_test(123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      1234567890,
      set_of!(),
      set_of!()
    );

    run_test(123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      1234567890,
      set_of!(),
      set_of!(333520145)
    );

    run_test(123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      333520145,
      set_of!(),
      set_of!(333520145)
    );

    run_test(123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      333520145,
      set_of!(333520145),
      set_of!(333520145)
    );

    run_test(123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      333520145,
      set_of!(333520145),
      set_of!(333520145)
    );

    run_test(333520145, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      333520145,
      set_of!(333520145, 333520391),
      set_of!(333520145, 333520391)
    );

    run_test(123, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      333859392,
      set_of!(),
      set_of!(333890765)
    );

    run_test(333890765, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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

    let post_parser_context = create_post_parser_context(
      1234,
      set_of!(),
      set_of!(333863078)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_dead_link_when_we_do_not_have_locally_cached_post() {
    let post_comment_raw = "<a href=\"#p333918351\" class=\"quotelink\">&gt;&gt;333918351</a><br>Because JOPs can just go to their \
    dedicated thread on /jp/. &gt;<span class=\"deadlink\">&gt;&gt;34511118</span>";
    let expected_parsed_comment = ">>333918351\nBecause JOPs can just go to their dedicated thread on /jp/. >>>34511118 (DEAD)";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333918351 }) },
      Spannable { start: 73, len: 17, spannable_data: SpannableData::Link(PostLink::Dead { post_no: 34511118 }) },
    ];

    let post_parser_context = create_post_parser_context(
      1234,
      set_of!(),
      set_of!(333918351)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_dead_link_when_we_have_locally_cached_post() {
    let post_comment_raw = "<a href=\"#p333918351\" class=\"quotelink\">&gt;&gt;333918351</a><br>Because JOPs can just go to their \
    dedicated thread on /jp/. &gt;<span class=\"deadlink\">&gt;&gt;34511118</span>";
    let expected_parsed_comment = ">>333918351\nBecause JOPs can just go to their dedicated thread on /jp/. >>>34511118";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333918351 }) },
      Spannable { start: 73, len: 10, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 34511118 }) },
    ];

    let post_parser_context = create_post_parser_context(
      1234,
      set_of!(),
      set_of!(333918351, 34511118)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_link_detection() {
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

    let expected_parsed_comment = "https://www.youtube.com/watch?v=57tu8AtKf9E
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

    let expected_spannables = vec![
      Spannable { start: 0, len: 43, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://www.youtube.com/watch?v=57tu8AtKf9E") }) },
      Spannable { start: 44, len: 47, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://boards.4channel.org/vg/thread/333979978") }) },
      Spannable { start: 97, len: 30, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://boards.4channel.org/v/") }) },
      Spannable { start: 129, len: 44, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("http://visual-novels-general.wikia.com/wiki/") }) },
      Spannable { start: 174, len: 39, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://sites.google.com/view/moechart/") }) },
      Spannable { start: 214, len: 35, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://files.catbox.moe/143by7.png") }) },
      Spannable { start: 258, len: 31, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://i.imgur.com/3CDmFQm.jpg") }) },
      Spannable { start: 298, len: 17, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("http://vndb.org/g") }) },
      Spannable { start: 316, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://pastebin.com/YTGdpqZL") }) },
      Spannable { start: 354, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://pastebin.com/YTGdpqZL") }) },
      Spannable { start: 384, len: 12, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("http://2.com") }) },
      Spannable { start: 397, len: 22, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://pastebin.com/1") }) },
      Spannable { start: 420, len: 26, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://w4534gerhnrh.com/2") }) },
      Spannable { start: 447, len: 22, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://pastebin.com/3") }) },
      Spannable { start: 471, len: 43, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://www.youtube.com/watch?v=57tu8AtKf9E") }) },
    ];

    let post_parser_context = create_post_parser_context(
      1234,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_cross_thread_link() {
    let post_comment_raw = "<a href=\"#p81423695\" class=\"quotelink\">&gt;&gt;81423695</a><br>We have one here with sound.<br>\
    <a href=\"//boards.4channel.org/wsg/thread/3849481#p3849481\" class=\"quotelink\" style=\"\">&gt;&gt;&gt;/wsg/3849481</a>";

    let expected_parsed_comment = ">>81423695\nWe have one here with sound.\n>>>/wsg/3849481 →";

    let expected_spannables = vec![
      Spannable { start: 0, len: 10, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 81423695 }) },
      Spannable { start: 40, len: 17, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: String::from("wsg"), thread_no: 3849481, post_no: 3849481 }) },
    ];

    let post_parser_context = create_post_parser_context(
      1234,
      set_of!(),
      set_of!(81423695)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_board_link_search_link_cross_thread_link() {
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

    let expected_parsed_comment = "\
    >Read the sticky: >>76759434 →\n\n\
    >GNU/Linux questions >>>/g/fglt\n\n\
    >Windows questions >>>/g/fwt\n\n\
    >PC building? >>>/g/pcbg\n\n\
    >Programming questions >>>/g/dpt\n\n\
    >Good, cheap, laptops >>>/g/tpg\n\n\
    >Cheap electronics >>>/g/csg\n\n\
    >Server questions >>>/g/hsg\n\n\
    >Buying headphones >>>/g/hpg\n\n\
    How to find/activate any version of Windows?\n\
    https://rentry.org/installwindows\n\n\
    Previous Thread >>81404563 →";

    let expected_spannables = vec![
      Spannable { start: 18, len: 12, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: String::from("g"), thread_no: 76759434, post_no: 76759434 }) },
      Spannable { start: 0, len: 30, spannable_data: SpannableData::GreenText },
      Spannable { start: 31, len: 22, spannable_data: SpannableData::GreenText },
      Spannable { start: 53, len: 10, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("fglt") }) },
      Spannable { start: 64, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 84, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("fwt") }) },
      Spannable { start: 94, len: 15, spannable_data: SpannableData::GreenText },
      Spannable { start: 109, len: 10, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("pcbg") }) },
      Spannable { start: 120, len: 24, spannable_data: SpannableData::GreenText },
      Spannable { start: 144, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("dpt") }) },
      Spannable { start: 154, len: 23, spannable_data: SpannableData::GreenText },
      Spannable { start: 177, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("tpg") }) },
      Spannable { start: 187, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 207, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("csg") }) },
      Spannable { start: 217, len: 19, spannable_data: SpannableData::GreenText },
      Spannable { start: 236, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("hsg") }) },
      Spannable { start: 246, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 266, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("hpg") }) },
      Spannable { start: 322, len: 33, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://rentry.org/installwindows") }) },
      Spannable { start: 373, len: 12, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: String::from("g"), thread_no: 81404563, post_no: 81404563 }) },
    ];

    let post_parser_context = create_post_parser_context(
      81425984,
      set_of!(),
      set_of!(81425984)
    );

    run_test(81425984, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_incorrectly_parsed_span_length_case() {
    let post_comment_raw = "<span class=\"quote\">&gt;Cinco de Mayo (pronounced [ˈsiŋko̞ ðe̞ ˈma̠ʝo̞] in Mexico, Spanish for &quot;Fifth of May&quot;) \
    is an annual celebration held on May 5. The date is observed to commemorate the Mexican Army&#039;s victory over the French Empire at the Battle of Puebla, \
    on May 5, 1862, under the leadership of General Ignacio Zaragoza.[1][2] The victory of the smaller Mexican force against a larger French force was \
    a boost to morale for the Mexicans. Zaragoza died months after the battle due to illness. A year after the battle, a larger French force defeated the \
    Mexican army at the Second Battle of Puebla, and Mexico City soon fell to the invaders.</span><br>what?";

    let expected_parsed_comment = ">Cinco de Mayo (pronounced [ˈsiŋko̞ ðe̞ ˈma̠ʝo̞] in Mexico, Spanish for \"Fifth of May\") \
    is an annual celebration held on May 5. The date is observed to commemorate the Mexican Army's victory over the French Empire \
    at the Battle of Puebla, on May 5, 1862, under the leadership of General Ignacio Zaragoza.[1][2] The victory of the smaller \
    Mexican force against a larger French force was a boost to morale for the Mexicans. Zaragoza died months after the battle due \
    to illness. A year after the battle, a larger French force defeated the Mexican army at the Second Battle of Puebla, and Mexico \
    City soon fell to the invaders.
what?";

    let expected_spannables = vec![
      Spannable { start: 0, len: 623, spannable_data: SpannableData::GreenText }
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_wbr_tag_removal_case() {
    let post_comment_raw = "<a href=\"#p221656514\" class=\"quotelink\">&gt;&gt;221656514</a>\
    <br>Be seeing you in the next rotation anons!<br>https://s1.desu-usergeneratedconten<wbr>t.xyz/a/image/1614/51/1614513969521<wbr>.png";

    let expected_parsed_comment = ">>221656514\nBe seeing you in the next rotation anons!\nhttps://s1.desu-usergeneratedcontent.xyz/a/image/1614/51/1614513969521.png";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 221656514 }) },
      Spannable { start: 54, len: 74, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://s1.desu-usergeneratedcontent.xyz/a/image/1614/51/1614513969521.png") }) },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!(221656514u64)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_target_blank_attribute_parsing() {
    let post_comment_raw = "<span class=\"quote\">&gt;Past thread:</span><br>https://desuarchive.org/aco/thread/<wbr>5189244<br>\
    <br><span class=\"quote\">&gt;Other CYOA Threads</span><br><a href=\"https://boards.4chan.org/search#/cyoag\" target=\"_blank\">\
    https://boards.4chan.org/search#/cy<wbr>oag</a><br><br><span class=\"quote\">&gt;/cyoag/&#039;s CYOA archives and related resources:</span>\
    <br>https://pastebin.com/vrqYhnpu<br>Includes - but is not limited to - personal archives of a number of authors, and an extensive \
    Allsync archive that has both SFW and NSFW CYOAs.<br>If you&#039;re looking for a specific CYOA, it&#039;s suggested that you check those first.";

    let expected_parsed_comment = ">Past thread:\nhttps://desuarchive.org/aco/thread/5189244\n\n>Other CYOA Threads\n\
    https://boards.4chan.org/search#/cyoag\n\n>/cyoag/\'s CYOA archives and related resources:\nhttps://pastebin.com/vrqYhnpu\n\
    Includes - but is not limited to - personal archives of a number of authors, and an extensive Allsync archive that has both SFW and NSFW CYOAs.\n\
    If you\'re looking for a specific CYOA, it\'s suggested that you check those first.";

    let expected_spannables = vec![
      Spannable { start: 0, len: 13, spannable_data: SpannableData::GreenText },
      Spannable { start: 14, len: 42, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://desuarchive.org/aco/thread/5189244") }) },
      Spannable { start: 57, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 78, len: 38, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://boards.4chan.org/search#/cyoag") }) },
      Spannable { start: 117, len: 48, spannable_data: SpannableData::GreenText },
      Spannable { start: 166, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://pastebin.com/vrqYhnpu") }) },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_unicode_japanese_text() {
    let post_comment_raw = "<a href=\"#p221655599\" class=\"quotelink\">&gt;&gt;221655599</a><br>Aaaaaaa the day can&#039;t be over yet<br><br>だれか !!!時よ止まれ,お願いします！！！";
    let expected_parsed_comment = ">>221655599\nAaaaaaa the day can\'t be over yet\n\nだれか !!!時よ止まれ,お願いします！！！";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 221655599 }) },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!(221655599u64)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_exif_toggle_case() {
    let post_comment_raw = "looks like autism and seasonal sales/releases are related, check the autism lows vs dslr highs. \
    seems that /p/haggots are confirmed retards and gearfagging is the seasonal temporary cure. LEL<br><br><a href=\"#p3878363\" \
    class=\"quotelink\">&gt;&gt;3878363</a><br>ah well, I was kinda drunk passing Vietnam to Cambodia river border, traveled for around 8 hours via a wooden boat. \
    good times<br><br><span class=\"abbr\">[EXIF data available. Click <a href=\"javascript:void(0)\" onclick=\"toggle('exif1620356074086')\">here</a> to show/hide.]</span><br>\
    <table class=\"exif\" id=\"exif1620356074086\"><tr><td colspan=\"2\"><b>Camera-Specific Properties:</b></td></tr><tr><td colspan=\"2\"><b></b></td>\
    </tr><tr><td>Camera Software</td><td>Android RP1A.200720.012.P615XXU4CUC3</td></tr><tr><td colspan=\"2\"><b></b></td></tr><tr><td colspan=\"2\"><b>\
    Image-Specific Properties:</b></td></tr><tr><td colspan=\"2\"><b></b></td></tr><tr><td>Image Width</td><td>2000</td></tr><tr><td>Image Height</td>\
    <td>1200</td></tr><tr><td>Image Orientation</td><td>Top, Left-Hand</td></tr><tr><td colspan=\"2\"><b></b></td></tr></table>";

    let expected_parsed_comment = "looks like autism and seasonal sales/releases are related, check the autism lows vs dslr highs. \
    seems that /p/haggots are confirmed retards and gearfagging is the seasonal temporary cure. LEL\n\n\
    >>3878363\nah well, I was kinda drunk passing Vietnam to Cambodia river border, traveled for around 8 hours via a wooden boat. good times\n\n\n\
    Camera-Specific Properties: \n\n\
    Camera Software Android RP1A.200720.012.P615XXU4CUC3 \n\n\
    Image-Specific Properties: \n\n\
    Image Width 2000 \n\
    Image Height 1200 \n\
    Image Orientation Top, Left-Hand \n\n";

    let expected_spannables = vec![
      Spannable { start: 193, len: 9, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 3878363 }) },
      Spannable { start: 331, len: 28, spannable_data: SpannableData::BoldText },
      Spannable { start: 416, len: 27, spannable_data: SpannableData::BoldText },
      Spannable { start: 331, len: 187, spannable_data: SpannableData::Monospace },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!(3878363u64)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_bold_tag() {
    let post_comment_raw = "More of her? <br><br><b style=\"color: red;\">(USER WAS BANNED FOR THIS POST)</b>";
    let expected_parsed_comment = "More of her? \n\n(USER WAS BANNED FOR THIS POST)";

    let expected_spannables = vec![
      Spannable { start: 14, len: 32, spannable_data: SpannableData::TextForegroundColorRaw { color_hex: "#FF0000".to_string() } },
      Spannable { start: 14, len: 32, spannable_data: SpannableData::BoldText },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_style_attr1() {
    let post_comment_raw = "More of her? <br><br><strong style=\"color: red;\">(USER WAS BANNED FOR THIS POST)</strong>";
    let expected_parsed_comment = "More of her? \n\n(USER WAS BANNED FOR THIS POST)";

    let expected_spannables = vec![
      Spannable { start: 14, len: 32, spannable_data: SpannableData::TextForegroundColorRaw { color_hex: "#FF0000".to_string() } },
      Spannable { start: 14, len: 32, spannable_data: SpannableData::BoldText },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_style_attr2() {
    let post_comment_raw = "1. You must check your #fortune in order to post in this thread.<span class=\"fortune\" style=\"color:#fd4d32\">\
    <br><br><b>Your fortune: Excellent Luck</b></span>";
    let expected_parsed_comment = "1. You must check your #fortune in order to post in this thread.\n\nYour fortune: Excellent Luck";

    let expected_spannables = vec![
      Spannable { start: 65, len: 29, spannable_data: SpannableData::BoldText },
      Spannable { start: 0, len: 94, spannable_data: SpannableData::TextForegroundColorRaw { color_hex: "#fd4d32".to_string() } },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_pre_tag() {
    let post_comment_raw = "<a href=\"/g/thread/81446291#p81478722\" class=\"quotelink\">&gt;&gt;81478722</a><br>nvm I guess I will just use<br>\
    <pre class=\"prettyprint\">ls -l | grep -i ^d.* | wc -l</pre><br>and write a function, shouldn&#039;t be long";

    let expected_parsed_comment = ">>81478722 →\nnvm I guess I will just use\nls -l | grep -i ^d.* | wc -l\nand write a function, shouldn\'t be long";

    let expected_spannables = vec![
      Spannable { start: 0, len: 12, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: "g".to_string(), thread_no: 81446291, post_no: 81478722 }) },
      Spannable { start: 40, len: 29, spannable_data: SpannableData::Monospace },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!(81478722u64)
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_cross_board_link() {
    let post_comment_raw = "All posting of untranslated visual novels belongs on <a href=\"//boards.4channel.org/jp/\" class=\"quotelink\">&gt;&gt;&gt;/jp/</a>\
    <br>E-celeb shitposting is not allowed.<br>";

    let expected_parsed_comment = "All posting of untranslated visual novels belongs on >>>/jp/\nE-celeb shitposting is not allowed.\n";

    let expected_spannables = vec![
      Spannable { start: 53, len: 7, spannable_data: SpannableData::Link(PostLink::BoardLink { board_code: "jp".to_string() }) },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_spannable_out_of_parsed_text_bounds() {
    let post_comment_raw = "#4137 ecclesia edition<br><br>Previous: <a href=\"/vg/thread/334945645#p334945645\" class=\"quotelink\">&gt;&gt;334945645</a>\
    <br><br><span class=\"quote\">&gt;Recommended Simulators</span><br>Automated:<br>●EDOPro (PC/Android). Download: https://projectignis.github.io/down<wbr>load.html<br>Manual:<br>\
    ●Duelingbook (online). Visit: https://www.duelingbook.com<br>Hosts, use the tag “/dng/” and the password “vidya”; on EDOPro, specify the server.<br><br>\
    <span class=\"quote\">&gt;Useful Links</span><br>Rulebook: http://www.yugioh-card.com/en/ruleb<wbr>ook/SD_RuleBook_EN_V10.pdf<br>\
    Wiki: https://yugipedia.com/wiki/Yugipedi<wbr>a<br>Probability Calculator: http://yugioh.party<br>Stock Market: http://yugiohprices.com<br>Database: https://www.db.yugioh-card.com<br><br>\
    <span class=\"quote\">&gt;Decklists</span><br>OCG: https://www.izazin.com/taikai/resul<wbr>ts?tag=遊戯王<br>TCG: http://yugiohtopdecks.com/decklists<wbr><br><br><span class=\"quote\">&gt;News</span>\
    <br>JP: http://blog.livedoor.jp/maxut<br>EN: https://ygorganization.com<br><br><span class=\"quote\">&gt;Upcoming Releases</span><br>OCG:<br>●Structure Deck: Cyber Style’s Successor (May 15)<br>\
    ●Duelist Pack: Gale Duelist Edition (May 22)<br>●Animation Chronicle 2021 (Jun 12)<br>●Structure Deck: Overlay Universe (Jun 26)<br>●Duel Royale Deck Set EX (Jul 10)<br>\
    ●Burst of Destiny (Jul 17)<br>●Structure Deck R: Lost Sanctuary (Aug 7)<br>TCG:<br>●Lightning Overdrive (Jun 4)<br>●OTS Tournament Pack 16 (Jun 10)<br>●Egyptian God Deck (Jun 11)<br>\
    ●King&#039;s Court (Jun 25)<br>●Legendary Duelists: Synchro Storm (Jul 14)<br>●Structure Deck: Cyber Strike (Jul 30)<br>●Dawn of Majesty (Aug 13)<br>\
    ●2021 Tin of Ancient Battles (Sep 17)<br><br><span class=\"quote\">&gt;Upcoming /dng/ Events</span><br>●/dng/ Cup (May 15, 1700 UTC): https://challonge.com/dngcup8<br>\
    ●Fisherman Locals (Jun 5, 1700 UTC): https://challonge.com/dngfisherman<br>●HAT Format (June 26, 1730 UTC): https://challonge.com/dngHAT<br>\
    ●/dng/ Battle City (June 27, 1600 UTC): https://challonge.com/dngbattlecity<wbr>";

    let expected_parsed_comment = "#4137 ecclesia edition\n\n\
    Previous: >>334945645 →\n\n\
    >Recommended Simulators\n\
    Automated:\n\
    ●EDOPro (PC/Android). Download: https://projectignis.github.io/download.html\n\
    Manual:\n\
    ●Duelingbook (online). Visit: https://www.duelingbook.com\n\
    Hosts, use the tag “/dng/” and the password “vidya”; on EDOPro, specify the server.\n\n\
    >Useful Links\n\
    Rulebook: http://www.yugioh-card.com/en/rulebook/SD_RuleBook_EN_V10.pdf\n\
    Wiki: https://yugipedia.com/wiki/Yugipedia\n\
    Probability Calculator: http://yugioh.party\n\
    Stock Market: http://yugiohprices.com\n\
    Database: https://www.db.yugioh-card.com\n\n\
    >Decklists\n\
    OCG: https://www.izazin.com/taikai/results?tag=遊戯王\n\
    TCG: http://yugiohtopdecks.com/decklists\n\n\
    >News\n\
    JP: http://blog.livedoor.jp/maxut\n\
    EN: https://ygorganization.com\n\n\
    >Upcoming Releases\n\
    OCG:\n\
    ●Structure Deck: Cyber Style’s Successor (May 15)\n\
    ●Duelist Pack: Gale Duelist Edition (May 22)\n\
    ●Animation Chronicle 2021 (Jun 12)\n\
    ●Structure Deck: Overlay Universe (Jun 26)\n\
    ●Duel Royale Deck Set EX (Jul 10)\n\
    ●Burst of Destiny (Jul 17)\n\
    ●Structure Deck R: Lost Sanctuary (Aug 7)\n\
    TCG:\n\
    ●Lightning Overdrive (Jun 4)\n\
    ●OTS Tournament Pack 16 (Jun 10)\n\
    ●Egyptian God Deck (Jun 11)\n\
    ●King\'s Court (Jun 25)\n\
    ●Legendary Duelists: Synchro Storm (Jul 14)\n\
    ●Structure Deck: Cyber Strike (Jul 30)\n\
    ●Dawn of Majesty (Aug 13)\n\
    ●2021 Tin of Ancient Battles (Sep 17)\n\n\
    >Upcoming /dng/ Events\n\
    ●/dng/ Cup (May 15, 1700 UTC): https://challonge.com/dngcup8\n\
    ●Fisherman Locals (Jun 5, 1700 UTC): https://challonge.com/dngfisherman\n\
    ●HAT Format (June 26, 1730 UTC): https://challonge.com/dngHAT\n\
    ●/dng/ Battle City (June 27, 1600 UTC): https://challonge.com/dngbattlecity";

    let expected_spannables = vec![
      Spannable { start: 34, len: 13, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: "vg".to_string(), thread_no: 334945645, post_no: 334945645 }) },
      Spannable { start: 48, len: 24, spannable_data: SpannableData::GreenText },
      Spannable { start: 120, len: 44, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://projectignis.github.io/download.html".to_string() }) },
      Spannable { start: 205, len: 27, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.duelingbook.com".to_string() }) },
      Spannable { start: 311, len: 14, spannable_data: SpannableData::GreenText },
      Spannable { start: 350, len: 61, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://www.yugioh-card.com/en/rulebook/SD_RuleBook_EN_V10.pdf".to_string() }) },
      Spannable { start: 418, len: 36, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://yugipedia.com/wiki/Yugipedia".to_string() }) },
      Spannable { start: 479, len: 19, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://yugioh.party".to_string() }) },
      Spannable { start: 513, len: 23, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://yugiohprices.com".to_string() }) },
      Spannable { start: 547, len: 30, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.db.yugioh-card.com".to_string() }) },
      Spannable { start: 564, len: 11, spannable_data: SpannableData::GreenText },
      Spannable { start: 595, len: 42, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.izazin.com/taikai/results?tag=".to_string() }) },
      Spannable { start: 652, len: 35, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://yugiohtopdecks.com/decklists".to_string() }) },
      Spannable { start: 668, len: 6, spannable_data: SpannableData::GreenText },
      Spannable { start: 699, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://blog.livedoor.jp/maxut".to_string() }) },
      Spannable { start: 733, len: 26, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://ygorganization.com".to_string() }) },
      Spannable { start: 740, len: 19, spannable_data: SpannableData::GreenText },
      Spannable { start: 1306, len: 23, spannable_data: SpannableData::GreenText },
      Spannable { start: 1415, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngcup8".to_string() }) },
      Spannable { start: 1484, len: 34, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngfisherman".to_string() }) },
      Spannable { start: 1554, len: 28, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngHAT".to_string() }) },
      Spannable { start: 1625, len: 35, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngbattlecity".to_string() }) },
    ];

    let post_parser_context = create_post_parser_context(
      1235,
      set_of!(),
      set_of!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  // TODO: Archive links (all supported archives)
  // TODO: Unicode text (Russian/some other?)
  // TODO: parse links like this one (https://boards.4chan.org/search#/cyoag) as global search shortcuts
}