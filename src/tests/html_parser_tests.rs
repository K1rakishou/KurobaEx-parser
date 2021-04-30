#[cfg(test)]
mod test_main {
  use crate::html_parser::parser::HtmlParser;

  #[test]
  fn html_parser_test_1() {
    let html = "Test<a href=\"#p333650561\" class=\"quotelink\">&gt;&gt;33365<wbr>0561</a><br><span class=\"quote\">&gt;what&#039;s the <wbr>best alternative</span><br>Reps";

    let html_parser = HtmlParser::new();
    let nodes = html_parser.parse(html).unwrap();

    let expected = "Test<a, href=\"#p333650561\", class=\"quotelink\">&gt;&gt;33365<wbr>0561<br><span, class=\"quote\">&gt;what&#039;s the <wbr>best alternative<br>Reps";
    let actual = html_parser.debug_concat_into_string(&nodes);

    assert_eq!(expected, actual);
  }

}