#[cfg(test)]
mod test_main {
  use crate::html_parser::parser::HtmlParser;

  #[test]
  fn html_parser_test_1() {
    let html = "Test<a href=\"#p333650561\" class=\"quotelink\">&gt;&gt;33365<wbr>0561</a><br><span class=\"quote\">&gt;what&#039;s the<wbr>best alternative</span><br>Reps";

    let html_parser = HtmlParser::new();
    let nodes = html_parser.parse(html).unwrap();

    let expected = r##"Test
<a, href=#p333650561, class=quotelink>
&gt;&gt;33365
<wbr>
0561
<br>
<span, class=quote>
&gt;what&#039;s the
<wbr>
best alternative
<br>
Reps
"##;

    let actual = html_parser.debug_concat_into_string(&nodes);
    assert_eq!(expected, actual);
  }

  #[test]
  fn html_parser_test_2() {
    let html = r#"<s><a class="linkify twitter" rel="noreferrer noopener" target="_blank" href="https://twitter.com/denonbu_eng/status/1388107521022468102">https://twitter.com/denonbu_eng/sta<wbr>tus/1388107521022468102</a><a class="embedder" href="javascript:;" data-key="Twitter" data-uid="denonbu_eng/status/1388107521022468102" data-options="undefined" data-href="https://twitter.com/denonbu_eng/status/1388107521022468102">(<span>un</span>embed)</a></s>"#;

    let html_parser = HtmlParser::new();
    let nodes = html_parser.parse(html).unwrap();

    let expected = r#"<s>
<a, class=linkify twitter, rel=noreferrer noopener, target=_blank, href=https://twitter.com/denonbu_eng/status/1388107521022468102>
https://twitter.com/denonbu_eng/sta
<wbr>
tus/1388107521022468102
<a, class=embedder, href=javascript:;, data-key=Twitter, data-uid=denonbu_eng/status/1388107521022468102, data-options=undefined, data-href=https://twitter.com/denonbu_eng/status/1388107521022468102>
(
<span>
un
embed)
"#;

    let actual = html_parser.debug_concat_into_string(&nodes);
    assert_eq!(expected, actual);
  }

}