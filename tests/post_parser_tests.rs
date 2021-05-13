#[cfg(test)]
mod test_main {
  use new_post_parser_lib::{PostParserContext, Spannable, PostRaw, PostParser, SpannableData, PostLink, set_of, Element, ThreadDescriptor, BoardDescriptor, SiteDescriptor, PostDescriptor};
  use std::collections::HashSet;

  fn create_post_parser_context(
    my_replies: HashSet<u64>,
    thread_posts: HashSet<u64>
  ) -> PostParserContext {
    PostParserContext::new(
      my_replies,
      thread_posts
    )
  }

  fn run_test(
    thread_id: u64,
    post_id: u64,
    post_parser_context: &PostParserContext,
    raw_comment: &str,
    expected_parsed_comment: &str,
    expected_spannables: &Vec<Spannable>
  ) {
    let thread_descriptor = ThreadDescriptor {
      board_descriptor: BoardDescriptor {
        site_descriptor: SiteDescriptor { site_name: "4chan".to_string() },
        board_code: "g".to_string()
      },
      thread_no: thread_id
    };

    let post_raw = PostRaw {
      post_descriptor: PostDescriptor {
        thread_descriptor,
        post_no: post_id,
        post_sub_no: 0u64
      },
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
        (actual_spannable.start + actual_spannable.len) <= parsed_text.chars().count(),
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
      set_of!(),
      set_of!()
    );

    run_test(1234567890, 123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_empty_tag_with_text() {
    let post_comment_raw = "Test1<b></b>Test2";
    let expected_parsed_comment = "Test1Test2";

    let expected_spannables = vec![];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!()
    );

    run_test(1234567890, 123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1234567890, 123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(333520145)
    );

    run_test(1234567890, 123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(333520145)
    );

    run_test(333520145, 123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(333520145),
      set_of!(333520145)
    );

    run_test(333520145, 123456780, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(333520145),
      set_of!(333520145)
    );

    run_test(333520145, 333520145, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(333520145, 333520391),
      set_of!(333520145, 333520391)
    );

    run_test(333520145, 123, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(333890765)
    );

    run_test(333859392, 333890765, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(333863078)
    );

    run_test(1234, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(333918351)
    );

    run_test(1234, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(333918351, 34511118)
    );

    run_test(1234, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1234, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(81423695)
    );

    run_test(1234, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(81425984)
    );

    run_test(81425984, 81425984, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(221656514u64)
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_unicode_japanese_text() {
    let post_comment_raw = "<a href=\"#p221655599\" class=\"quotelink\">&gt;&gt;221655599</a><br>Aaaaaaa the day can&#039;t be over yet<br><br>だれか !!!時よ止まれ,お願いします！！！";
    let expected_parsed_comment = ">>221655599\nAaaaaaa the day can\'t be over yet\n\nだれか !!!時よ止まれ,お願いします！！！";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 221655599 }) },
    ];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!(221655599u64)
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(3878363u64)
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!(81478722u64)
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
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
      Spannable { start: 116, len: 44, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://projectignis.github.io/download.html".to_string() }) },
      Spannable { start: 199, len: 27, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.duelingbook.com".to_string() }) },
      Spannable { start: 311, len: 14, spannable_data: SpannableData::GreenText },
      Spannable { start: 336, len: 61, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://www.yugioh-card.com/en/rulebook/SD_RuleBook_EN_V10.pdf".to_string() }) },
      Spannable { start: 404, len: 36, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://yugipedia.com/wiki/Yugipedia".to_string() }) },
      Spannable { start: 465, len: 19, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://yugioh.party".to_string() }) },
      Spannable { start: 499, len: 23, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://yugiohprices.com".to_string() }) },
      Spannable { start: 533, len: 30, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.db.yugioh-card.com".to_string() }) },
      Spannable { start: 564, len: 11, spannable_data: SpannableData::GreenText },
      Spannable { start: 581, len: 45, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.izazin.com/taikai/results?tag=遊戯王".to_string() }) },
      Spannable { start: 632, len: 35, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://yugiohtopdecks.com/decklists".to_string() }) },
      Spannable { start: 668, len: 6, spannable_data: SpannableData::GreenText },
      Spannable { start: 679, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "http://blog.livedoor.jp/maxut".to_string() }) },
      Spannable { start: 713, len: 26, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://ygorganization.com".to_string() }) },
      Spannable { start: 740, len: 19, spannable_data: SpannableData::GreenText },
      Spannable { start: 1306, len: 23, spannable_data: SpannableData::GreenText },
      Spannable { start: 1361, len: 29, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngcup8".to_string() }) },
      Spannable { start: 1428, len: 34, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngfisherman".to_string() }) },
      Spannable { start: 1496, len: 28, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngHAT".to_string() }) },
      Spannable { start: 1565, len: 35, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngbattlecity".to_string() }) },
    ];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_unicode_char_at_start() {
    let post_comment_raw = "●/dng/ Battle City (June 27, 1600 UTC): https://challonge.com/dngbattlecity";
    let expected_parsed_comment = "●/dng/ Battle City (June 27, 1600 UTC): https://challonge.com/dngbattlecity";

    let expected_spannables = vec![
      Spannable { start: 40, len: 35, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://challonge.com/dngbattlecity".to_string() }) },
    ];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_unicode_char_at_end() {
    let post_comment_raw = "/dng/ Battle City (June 27, 1600 UTC): https://www.izazin.com/taikai/results?tag=遊戯王";
    let expected_parsed_comment = "/dng/ Battle City (June 27, 1600 UTC): https://www.izazin.com/taikai/results?tag=遊戯王";

    let expected_spannables = vec![
      Spannable { start: 39, len: 45, spannable_data: SpannableData::Link(PostLink::UrlLink { link: "https://www.izazin.com/taikai/results?tag=遊戯王".to_string() }) },
    ];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_app_theme_json() {
    let post_comment_raw = "{<br> &quot;name&quot;: &quot;Kuromakaba Light&quot;,<br> &quot;is_light_theme&quot;: true,<br> \
    &quot;light_nav_bar&quot;: true,<br> &quot;light_status_bar&quot;: true,<br> &quot;accent_color&quot;: &quot;#f3630e&quot;,<br> \
    &quot;primary_color&quot;: &quot;#000000&quot;,<br> &quot;back_color&quot;: &quot;#e2e2e2&quot;,<br> \
    &quot;post_details_color&quot;: &quot;#6a6a6a&quot;,<br> &quot;post_highlight_quote_color&quot;: &quot;#f3630e&quot;,<br> \
    &quot;post_highlighted_color&quot;: &quot;#f1f1f1&quot;,<br> &quot;post_inline_quote_color&quot;: &quot;#899918&quot;,<br> \
    &quot;post_link_color&quot;: &quot;#f3630e&quot;,<br> &quot;post_name_color&quot;: &quot;#6a6a6a&quot;,<br> \
    &quot;post_quote_color&quot;: &quot;#f3630e&quot;,<br> &quot;post_saved_reply_color&quot;: &quot;#f1f1f1&quot;,<br> \
    &quot;post_spoiler_color&quot;: &quot;#c6c6c6&quot;,<br> &quot;post_spoiler_reveal_text_color&quot;: &quot;#303030&quot;,<br> \
    &quot;post_subject_color&quot;: &quot;#134b6b&quot;,<br> &quot;post_unseen_label_color&quot;: &quot;#ea8162&quot;,<br> \
    &quot;text_color_hint&quot;: &quot;#6a6a6a&quot;,<br> &quot;text_color_primary&quot;: &quot;#474747&quot;,<br> \
    &quot;text_color_secondary&quot;: &quot;#6a6a6a&quot;,<br> &quot;bookmark_counter_has_replies_color<wbr>&quot;: &quot;#f3630e&quot;,<br> \
    &quot;bookmark_counter_normal_color&quot;: &quot;#474747&quot;,<br> &quot;bookmark_counter_not_watching_colo<wbr>r&quot;: &quot;#6a6a6a&quot;,<br> \
    &quot;divider_color&quot;: &quot;#c6c6c6&quot;,<br> &quot;error_color&quot;: &quot;#ff0000&quot;<br>}";

    let expected_parsed_comment = "{\n \"name\": \"Kuromakaba Light\",\n \"is_light_theme\": true,\n \"light_nav_bar\": true,\n \"light_status_bar\": true,\n \"accent_color\": \"#f3630e\",\n \"primary_color\": \"#000000\",\n \"back_color\": \"#e2e2e2\",\n \"post_details_color\": \"#6a6a6a\",\n \"post_highlight_quote_color\": \"#f3630e\",\n \"post_highlighted_color\": \"#f1f1f1\",\n \"post_inline_quote_color\": \"#899918\",\n \"post_link_color\": \"#f3630e\",\n \"post_name_color\": \"#6a6a6a\",\n \"post_quote_color\": \"#f3630e\",\n \"post_saved_reply_color\": \"#f1f1f1\",\n \"post_spoiler_color\": \"#c6c6c6\",\n \"post_spoiler_reveal_text_color\": \"#303030\",\n \"post_subject_color\": \"#134b6b\",\n \"post_unseen_label_color\": \"#ea8162\",\n \"text_color_hint\": \"#6a6a6a\",\n \"text_color_primary\": \"#474747\",\n \"text_color_secondary\": \"#6a6a6a\",\n \"bookmark_counter_has_replies_color\": \"#f3630e\",\n \"bookmark_counter_normal_color\": \"#474747\",\n \"bookmark_counter_not_watching_color\": \"#6a6a6a\",\n \"divider_color\": \"#c6c6c6\",\n \"error_color\": \"#ff0000\"\n}";

    let expected_spannables = vec![
      Spannable { start: 0, len: 931, spannable_data: SpannableData::ThemeJson { theme_name: "Kuromakaba Light".to_string(), is_light_theme: true } },
    ];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_app_theme_json_inside_code_tags() {
    let post_comment_raw = "<pre class=\"prettyprint\">{<br>&quot;name&quot;: &quot;Kuromakaba Light&quot;,<br>&quot;is_light_theme&quot;: true,<br>\
    &quot;light_nav_bar&quot;: true,<br>&quot;light_status_bar&quot;: true,<br>&quot;accent_color&quot;: &quot;#f3630e&quot;,<br>&quot;primary_color&quot;: &quot;#000000&quot;,<br>\
    &quot;back_color&quot;: &quot;#e2e2e2&quot;,<br>&quot;post_details_color&quot;: &quot;#6a6a6a&quot;,<br>&quot;post_highlight_quote_color&quot;: &quot;#f3630e&quot;,<br>\
    &quot;post_highlighted_color&quot;: &quot;#f1f1f1&quot;,<br>&quot;post_inline_quote_color&quot;: &quot;#899918&quot;,<br>&quot;post_link_color&quot;: &quot;#f3630e&quot;,<br>\
    &quot;post_name_color&quot;: &quot;#6a6a6a&quot;,<br>&quot;post_quote_color&quot;: &quot;#f3630e&quot;,<br>&quot;post_saved_reply_color&quot;: &quot;#f1f1f1&quot;,<br>\
    &quot;post_spoiler_color&quot;: &quot;#c6c6c6&quot;,<br>&quot;post_spoiler_reveal_text_color&quot;: &quot;#303030&quot;,<br>&quot;post_subject_color&quot;: &quot;#134b6b&quot;,<br>\
    &quot;post_unseen_label_color&quot;: &quot;#ea8162&quot;,<br>&quot;text_color_hint&quot;: &quot;#6a6a6a&quot;,<br>&quot;text_color_primary&quot;: &quot;#474747&quot;,<br>\
    &quot;text_color_secondary&quot;: &quot;#6a6a6a&quot;,<br>&quot;bookmark_counter_has_replies_color<wbr>&quot;: &quot;#f3630e&quot;,<br>\
    &quot;bookmark_counter_normal_color&quot;: &quot;#474747&quot;,<br>&quot;bookmark_counter_not_watching_colo<wbr>r&quot;: &quot;#6a6a6a&quot;,<br>\
    &quot;divider_color&quot;: &quot;#c6c6c6&quot;,<br>&quot;error_color&quot;: &quot;#ff0000&quot;<br>}</pre>";

    let expected_parsed_comment = "{\n\"name\": \"Kuromakaba Light\",\n\"is_light_theme\": true,\n\"light_nav_bar\": true,\n\"light_status_bar\": true,\n\"accent_color\": \"#f3630e\",\n\"primary_color\": \"#000000\",\n\"back_color\": \"#e2e2e2\",\n\"post_details_color\": \"#6a6a6a\",\n\"post_highlight_quote_color\": \"#f3630e\",\n\"post_highlighted_color\": \"#f1f1f1\",\n\"post_inline_quote_color\": \"#899918\",\n\"post_link_color\": \"#f3630e\",\n\"post_name_color\": \"#6a6a6a\",\n\"post_quote_color\": \"#f3630e\",\n\"post_saved_reply_color\": \"#f1f1f1\",\n\"post_spoiler_color\": \"#c6c6c6\",\n\"post_spoiler_reveal_text_color\": \"#303030\",\n\"post_subject_color\": \"#134b6b\",\n\"post_unseen_label_color\": \"#ea8162\",\n\"text_color_hint\": \"#6a6a6a\",\n\"text_color_primary\": \"#474747\",\n\"text_color_secondary\": \"#6a6a6a\",\n\"bookmark_counter_has_replies_color\": \"#f3630e\",\n\"bookmark_counter_normal_color\": \"#474747\",\n\"bookmark_counter_not_watching_color\": \"#6a6a6a\",\n\"divider_color\": \"#c6c6c6\",\n\"error_color\": \"#ff0000\"\n}";

    let expected_spannables = vec![
      Spannable { start: 0, len: 904, spannable_data: SpannableData::Monospace },
      Spannable { start: 0, len: 904, spannable_data: SpannableData::ThemeJson { theme_name: "Kuromakaba Light".to_string(), is_light_theme: true } },
    ];

    let post_parser_context = create_post_parser_context(
      set_of!(),
      set_of!()
    );

    run_test(1235, 1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  // A general for the discussion of all non-x86 ISAs (RISC-V, SPARC, m68k, PA-RISC, MIPS, Itanium, PowerPC, etc.) retro and modern.<br><br>M68K:<br>http://m68k.info/<br>http://www.apollo-core.com/index.ht<wbr>m<br>https://thebrewingacademy.com/colle<wbr>ctions/atari-st-ste-mega<br>http://www.easy68k.com/paulrsm/<br>https://github.com/grovdata/Amiga_S<wbr>ources<br><br>MIPS:<br>https://www.embeddedplanet.com/prod<wbr>ucts_list/cavium-octeon-iii-develop<wbr>ment-board/<br>https://elinux.org/MIPS_Creator_CI2<wbr>0<br>http://www.sgistuff.net/software/ir<wbr>ixintro/index.html<br>https://sgi.neocities.org/<br><br>SuperH:<br>https://www.apnet.co.jp/product/ms1<wbr>04/ms104-sh4.html<br><br>Z80:<br>http://www.pc1500.com/<br>https://www.kickstarter.com/project<wbr>s/spectrumnext/zx-spectrum-next-iss<wbr>ue-2<br><br>6502:<br>http://6502.org/<br>https://gist.github.com/jblang/a397<wbr>48b3b0d3ceba05cbb92d0c56b3b2<br>https://www.commodorecomputerclub.c<wbr>om/resources/<br>http://home-2002.code-cop.org/c64/<br><br>RISC-V:<br>https://beagleboard.org/beaglev<br>https://bellard.org/tinyemu/<br>https://www.sifive.com/boards/hifiv<wbr>e-unmatched<br><br>SPARC:<br>https://sparc.org/<br><br>POWER/PowerPC:<br>https://www.nxp.com/design/qoriq-de<wbr>veloper-resources/qoriq-t2080-devel<wbr>opment-board:T2080RDB<br>https://www.powerpc-notebook.org<br>https://raptorcs.com/content/BK1SD1<wbr>/intro.html<br><br>VAX:<br>https://github.com/simh/simh<br>http://oboguev.net/vax_mp/<br><br>Alpha:<br>https://github.com/lenticularis39/a<wbr>xpbox<br><br>Multi-system FPGA:<br>https://github.com/mist-devel/mist-<wbr>board/wiki<br>https://github.com/MiSTer-devel/Mai<wbr>n_MiSTer/wiki<br><br>Misc.:<br>http://anycpu.org<br>https://opencores.org/<br><br>More:<br>https://wiki.installgentoo.com/wiki<wbr>//aig/_Alternative_ISA_General<br>https://mega.nz/file/0PplHSyL#eK_f2<wbr>ZSc2f0E8_RLUGz9nVn40myXhyiRDMU_FhgO<wbr>2wk<br><br>Previous thread: <a href=\"/g/thread/81521490#p81521490\" class=\"quotelink\">&gt;&gt;81521490</a>

  // TODO: some MEGA links that we are not parsing correctly + another kind of search links
  // Mahou Shoujo Edition<br><br>Previous Thread: <a href=\"/vg/thread/335153273#p335153273\" class=\"quotelink\">&gt;&gt;335153273</a><br><br>2D Hentai games (Hentai RPG, Violated Heroine, H-Action Games) General /hgg2d/<br>For searching purposes: /vhg/<br><br>UNLESS EXPLICITLY STATED ASSUME THERE&#039;S NO TRANSLATION AND PLEASE READ THE PASTEBINS BEFORE ASKING QUESTIONS<br><br><span class=\"quote\">&gt;NEWCOMERS GUIDE: (PLEASE READ BEFORE POSTING)</span><br>https://ipfs.infura.io/ipfs/QmYspPo<wbr>kmbzYRozmGMWEU9nNUVqCbQZUNPbBWfFSgB<wbr>gD7W<br>UPDATE YOUR ARCHIVER BEFORE COMPLAINING ABOUT CORRUPT DOWNLOADS<br><br><span class=\"quote\">&gt;/hgg2d/ Game Archive - IPFS Edition</span><br>https://ipfs.io/ipns/k2k4r8l7mxpi57<wbr>sotykoy5f5ucakg0dr0ib0avmyjhwmofkvp<wbr>fhfd510/bin.html<br><br><span class=\"quote\">&gt;Recommended Games (outdated):</span><br><span class=\"quote\">&gt;New Recommended Games Bin:</span><br><span class=\"quote\">&gt;/hgg2d/ Gotw Catalog:</span><br>Refer to the newcomers guide link<br><br>IF YOU&#039;RE PLANNING TO PLAY RPGMAKER GAMES, MAKE SURE YOU HAVE THE RIGHT RTP FOR THEM DON&#039;T BOTHER THE THREAD WITH RGSSWHATEVER.dll AND MISSING FILES ERRORS<br>https://tkool.jp/products/rtp.html<br><br><span class=\"quote\">&gt;Nifty translation tool by Anon:</span><br>https://www.mediafire.com/file/ov74<wbr>ltf4cgpji59/japReader-1.2.zip/file<br><br>HOW TO PLAY THESE GAMES IN LINUX<br>https://pastebin.com/21Yi6vnt<br><br>RGSS DECRYPTER FOR APPLYING PATCHES<br>http://www50.zippyshare.com/v/17MCS<wbr>5Bb/file.html<br>If this can&#039;t extract some files then try:<br>https://github.com/usagirei/RGSS-De<wbr>cryptor<br><br>RJ CODE QUICKLINK SCRIPT (Updated to include the RE version changes)<br>https://hgg2d.github.io/<br><br>RJ Gallery SCRIPT<br>https://ipfs.infura.io/ipfs/QmX2wWM<wbr>ed1oD5RntcTqBFXCDGX4mGgNKfj1v6pHQtE<wbr>B62K/DL%20Site%20Previewer.html<br><br>DLsite partial interface translation script<br>https://github.com/Zero-G-Sys/DLStr<wbr>ans<br><br>VH Specific<br><span class=\"quote\">&gt;latest VH translation</span><br>https://mega.nz/#F!F9ZyVSLY!6U0Tlvb<wbr>W88UFAynZ3pxJBg<br><span class=\"quote\">&gt;outdated mega</span><br>https://mega.nz/#F!0ChlQYJa!918hNc-<wbr>SzjigT-yBqiaidw<br><br>Installation: http://wiki.anime-sharing.com/hgame<wbr>s/index.php?title=VH<br>FAQ: http://pastebin.com/ZtDHt64k<br><br>Take all discussion of fetishes outside of the context of video games to <a href=\"//boards.4chan.org/h/\" class=\"quotelink\">&gt;&gt;&gt;/h/</a><br>Take all discussion of VNs to <a href=\"//boards.4channel.org/vg/catalog#s=vn%2F\" class=\"quotelink\">&gt;&gt;&gt;/vg/vn/</a> or <a href=\"//boards.4channel.org/vg/catalog#s=hgg%2F\" class=\"quotelink\">&gt;&gt;&gt;/vg/hgg/</a><br><br>Please wait for page 10 before making a new thread

  // TODO: comment with two [code] tags
  // I have no fucking clue why this works. I fucked up when I was making the form and put the submit/clear button outside.<br>If I try to place them back in the form tag, the new &lt;li&gt; shows up for a split second then disappears.<br><br><pre class=\"prettyprint\">    &lt;div id=&quot;to-do_list&quot;&gt;<br>    &lt;form&gt;<br>      &lt;input type=&quot;text&quot; id=&quot;to-do-item&quot; name=&quot;to-do_item&quot;&gt;<br>    &lt;/form&gt;<br>      &lt;input type=&quot;submit&quot; id=&quot;to-do-submit&quot; value=&quot;Add to list&quot;&gt;<br>      &lt;button id=&quot;clear&quot; onclick=&quot;removed()&quot;&gt;Clear All&lt;/button&gt;<br>    &lt;/div&gt;&lt;!--to-do_list--&gt;<br></pre><br><br><pre class=\"prettyprint\">    &lt;script&gt;<br>      &quot;use strict&quot;;<br>      let to_do_list_ol = document.getElementById(&quot;to-do_list<wbr>-ol&quot;);<br>      let input = document.getElementById(&quot;to-do-item<wbr>&quot;);<br>      let submit = document.getElementById(&quot;to-do-subm<wbr>it&quot;);<br>      submit.onclick = function(){<br>        let li_input = document.createElement(&#039;li&#039;); <br>        li_input.innerHTML = (`${input.value}&lt;button class=&quot;delete&quot; onclick=&quot;removeOne(this)&quot;&gt;Delete&lt;/b<wbr>utton&gt;`);<br>        to_do_list_ol.append(li_input); <br>      };<br><br>      let clear = document.getElementById(&quot;clear&quot;); <br><br>      function removed(){<br>        let li_items = document.querySelectorAll(&#039;li&#039;);<br>          for(let i of li_items){<br>            i.remove(); <br>          }<br>          alert(&#039;removed&#039;);<br>        };<br><br>        function removeOne(obj){ <br>          let li_obj = obj.closest(&#039;li&#039;); <br>          li_obj.remove();<br>        }<br><br>    &lt;/script&gt;<br>  &lt;/body&gt;<br>&lt;/html&gt;<br></pre>

  // TODO: Replace "http" with "https"
  // TODO: Archive links (all supported archives)
  // TODO: Unicode text (Russian/some other?)
  // TODO: parse links like this one (https://boards.4chan.org/search#/cyoag) as global search shortcuts
}