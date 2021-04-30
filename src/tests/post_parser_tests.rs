#[cfg(test)]
mod test_main {
  use crate::PostRaw;
  use crate::parser::parser::PostParser;
  use crate::comment_parser::parser::Spannable;
  use crate::comment_parser::parser::SpannableData;
  use crate::comment_parser::parser::PostLink;

  fn run_test(raw_comment: &str, expected_parsed_comment: &str, expected_spannables: &Vec<Spannable>) {
    let post_raw = PostRaw {
      com: Option::Some(String::from(raw_comment))
    };

    let post_parser = PostParser::new();
    let post_comment_parsed = post_parser.parse_post(&post_raw).post_comment_parsed.unwrap();
    let spannables = post_comment_parsed.spannables;

    assert_eq!(expected_parsed_comment, post_comment_parsed.comment_text.as_str());
    assert_eq!(expected_spannables.len(), spannables.len());

    for index in 0 .. spannables.len() {
      assert_eq!(expected_spannables[index], spannables[index]);
    }
  }

  #[test]
  fn post_parser_test_1() {
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>\
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;\
But Tomoya knows that. As soon as she becomes the student council president their relationship becomes a political issue with teachers telling her to break up with him,\
(not because she has a boyfriend, but because it&#039;s him specifically), so his perception of the relationship is realistic.\
She is incapable of achieving her goal while dating him, but she holds their relationship equal with her goal and so cannot end it herself,\
which is why he dumps her, so that she&#039;ll be able to achieve it. Stop talking about a game you haven&#039;t even read.\
Also the ending in the anime OVA is not quite the same as the game, so having seen that alone doesn&#039;t quality you to have anything to say about this.";
    let expected_parsed_comment = ">>333520145
>>333520391
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by \"dragging her down.\"\
But Tomoya knows that. As soon as she becomes the student council president their relationship becomes a political issue with teachers telling her to break up with him,\
(not because she has a boyfriend, but because it's him specifically), so his perception of the relationship is realistic.\
She is incapable of achieving her goal while dating him, but she holds their relationship equal with her goal and so cannot end it herself,\
which is why he dumps her, so that she'll be able to achieve it. Stop talking about a game you haven't even read.\
Also the ending in the anime OVA is not quite the same as the game, so having seen that alone doesn't quality you to have anything to say about this.";

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 12, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520391 }) }
    ];

    run_test(post_comment_raw, expected_parsed_comment, &expected_spannables);
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