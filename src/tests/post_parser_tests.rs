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
    let post_comment_raw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;
But Tomoya knows that. As soon as she becomes the student council president their relationship becomes a political issue with teachers telling her to break up with him,
(not because she has a boyfriend, but because it&#039;s him specifically), so his perception of the relationship is realistic.
She is incapable of achieving her goal while dating him, but she holds their relationship equal with her goal and so cannot end it herself,
which is why he dumps her, so that she&#039;ll be able to achieve it. Stop talking about a game you haven&#039;t even read.
Also the ending in the anime OVA is not quite the same as the game, so having seen that alone doesn&#039;t quality you to have anything to say about this.";
    let expected_parsed_comment = r#">>333520145
>>333520391
Feel free to tell me specifically what I'm wrong about. I'll take one thing he says: that Tomoya is behaving negatively by "dragging her down."
But Tomoya knows that. As soon as she becomes the student council president their relationship becomes a political issue with teachers telling her to break up with him,
(not because she has a boyfriend, but because it's him specifically), so his perception of the relationship is realistic.
She is incapable of achieving her goal while dating him, but she holds their relationship equal with her goal and so cannot end it herself,
which is why he dumps her, so that she'll be able to achieve it. Stop talking about a game you haven't even read.
Also the ending in the anime OVA is not quite the same as the game, so having seen that alone doesn't quality you to have anything to say about this."#;

    let expected_spannables = vec![
      Spannable { start: 0, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520145 }) },
      Spannable { start: 12, len: 11, spannable_data: SpannableData::Link(PostLink::Quote { post_no: 333520391 }) }
    ];

    run_test(post_comment_raw, expected_parsed_comment, &expected_spannables);
  }

}