#[cfg(test)]
mod test_main {
  use crate::html_parser::parser::HtmlParser;

  #[test]
  fn html_parser_test_1() {
    let html = "Reps<a href=\"#p333650561\" class=\"quotelink\">&gt;&gt;333650561</a><br><span class=\"quote\">&gt;what&#039;s the best alternative</span><br>Reps";

    let html_parser = HtmlParser::new();
    html_parser.parse(html);
  }

}