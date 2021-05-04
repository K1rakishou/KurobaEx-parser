#[cfg(test)]
mod test_main {
  use new_post_parser_lib::{PostParserContext, Spannable, PostRaw, PostParser, SpannableData, PostLink, set};
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
      1234567890,
      set!(),
      set!()
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

    let post_parser_context = PostParserContext::new(
      1234567890,
      set!(),
      set!(333520145)
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

    let post_parser_context = PostParserContext::new(
      333520145,
      set!(),
      set!(333520145)
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

    let post_parser_context = PostParserContext::new(
      333520145,
      set!(333520145),
      set!(333520145)
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

    let post_parser_context = PostParserContext::new(
      333520145,
      set!(333520145),
      set!(333520145)
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

    let post_parser_context = PostParserContext::new(
      333520145,
      set!(333520145, 333520391),
      set!(333520145, 333520391)
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

    let post_parser_context = PostParserContext::new(
      333859392,
      set!(),
      set!(333890765)
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

    let post_parser_context = PostParserContext::new(
      1234,
      set!(),
      set!(333863078)
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

    let post_parser_context = PostParserContext::new(
      1234,
      set!(),
      set!(333918351)
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

    let post_parser_context = PostParserContext::new(
      1234,
      set!(),
      set!(333918351, 34511118)
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

    let post_parser_context = PostParserContext::new(
      1234,
      set!(),
      set!()
    );

    run_test(1235, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  #[test]
  fn post_parser_test_cross_thread_link() {
    let post_comment_raw = "<a href=\"#p81423695\" class=\"quotelink\">&gt;&gt;81423695</a><br>We have one here with sound.<br>\
    <a href=\"//boards.4channel.org/wsg/thread/3849481#p3849481\" class=\"quotelink\" style=\"\">&gt;&gt;&gt;/wsg/3849481</a>";

    let expected_parsed_comment = ">>81423695\nWe have one here with sound.\n>>>/wsg/3849481";

    let expected_spannables = vec![
      Spannable {
        start: 0,
        len: 10,
        spannable_data: SpannableData::Link(PostLink::Quote { post_no: 81423695 })
      },
      Spannable {
        start: 40,
        len: 15,
        spannable_data: SpannableData::Link(PostLink::ThreadLink {
          board_code: String::from("wsg"),
          thread_no: 3849481,
          post_no: 3849481
        })
      },
    ];

    let post_parser_context = PostParserContext::new(
      1234,
      set!(),
      set!(81423695)
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
    >Read the sticky: >>76759434\n\n\
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
    Previous Thread >>81404563";

    let expected_spannables = vec![
      Spannable { start: 18, len: 10, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: String::from("g"), thread_no: 76759434, post_no: 76759434 }) },
      Spannable { start: 0, len: 28, spannable_data: SpannableData::GreenText },
      Spannable { start: 29, len: 22, spannable_data: SpannableData::GreenText },
      Spannable { start: 51, len: 10, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("fglt") }) },
      Spannable { start: 62, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 82, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("fwt") }) },
      Spannable { start: 92, len: 15, spannable_data: SpannableData::GreenText },
      Spannable { start: 107, len: 10, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("pcbg") }) },
      Spannable { start: 118, len: 24, spannable_data: SpannableData::GreenText },
      Spannable { start: 142, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("dpt") }) },
      Spannable { start: 152, len: 23, spannable_data: SpannableData::GreenText },
      Spannable { start: 175, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("tpg") }) },
      Spannable { start: 185, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 205, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("csg") }) },
      Spannable { start: 215, len: 19, spannable_data: SpannableData::GreenText },
      Spannable { start: 234, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("hsg") }) },
      Spannable { start: 244, len: 20, spannable_data: SpannableData::GreenText },
      Spannable { start: 264, len: 9, spannable_data: SpannableData::Link(PostLink::SearchLink { board_code: String::from("g"), search_query: String::from("hpg") }) },
      Spannable { start: 320, len: 33, spannable_data: SpannableData::Link(PostLink::UrlLink { link: String::from("https://rentry.org/installwindows") }) },
      Spannable { start: 371, len: 10, spannable_data: SpannableData::Link(PostLink::ThreadLink { board_code: String::from("g"), thread_no: 81404563, post_no: 81404563 }) },
    ];

    let post_parser_context = PostParserContext::new(
      81425984,
      set!(),
      set!(81425984)
    );

    run_test(81425984, &post_parser_context, post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

  // TODO: BoardLink
  // TODO: SearchLink
  // TODO: Unicode text (Japasene/Russian/some other?)

  // Bunch of cross-thread (dead and alive) links
  // Breakfast Baki, Maximum Tournament-hen.<br>The only rule is no weapons. Anything else goes.<br>OP for the finals: https://www.youtube.com/watch?v=9PY<wbr>FIgOaWz0<br><br>Last time: A flashback to Yuujiro&#039;s past in Vietnam.<br>Volume 21: <span class=\"deadlink\">&gt;&gt;220319165</span><br>Volume 22: <span class=\"deadlink\">&gt;&gt;220380103</span><br>Volume 23: <span class=\"deadlink\">&gt;&gt;220441774</span><br>Volume 24: <span class=\"deadlink\">&gt;&gt;220501577</span><br>Volume 25: <span class=\"deadlink\">&gt;&gt;220559839</span><br>Volume 26: <span class=\"deadlink\">&gt;&gt;220623109</span><br>Volume 27: <span class=\"deadlink\">&gt;&gt;220684674</span><br>Volume 28: <span class=\"deadlink\">&gt;&gt;220748482</span><br>Volume 29: <span class=\"deadlink\">&gt;&gt;220805232</span><br>Volume 30: <span class=\"deadlink\">&gt;&gt;220862935</span><br>Volume 31: <span class=\"deadlink\">&gt;&gt;220924399</span><br>Volume 32: <span class=\"deadlink\">&gt;&gt;220981838</span><br>Volume 33: <span class=\"deadlink\">&gt;&gt;221038807</span><br>Volume 34: <a href=\"/a/thread/221101420#p221101420\" class=\"quotelink\">&gt;&gt;221101420</a><br>Volume 35: <a href=\"/a/thread/221160344#p221160344\" class=\"quotelink\">&gt;&gt;221160344</a><br>Volume 36: <a href=\"/a/thread/221218747#p221218747\" class=\"quotelink\">&gt;&gt;221218747</a><br>Volume 37: <a href=\"/a/thread/221275313#p221275313\" class=\"quotelink\">&gt;&gt;221275313</a><br>Volume 38: <a href=\"/a/thread/221332969#p221332969\" class=\"quotelink\">&gt;&gt;221332969</a><br>Volume 39: <a href=\"/a/thread/221386198#p221386198\" class=\"quotelink\">&gt;&gt;221386198</a><br>Volume 40: <a href=\"/a/thread/221443978#p221443978\" class=\"quotelink\">&gt;&gt;221443978</a><br>https://archive.wakarimasen.moe/a/s<wbr>earch/subject/Storytime%3A%20Grappl<wbr>er%20Baki/
}