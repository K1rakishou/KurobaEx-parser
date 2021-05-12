use crate::{Spannable, SpannableData};
use regex::Regex;

lazy_static! {
  static ref THEME_JSON_KEYS: Vec<&'static str> = vec!["\"name\"", "\"is_light_theme\"", "\"light_nav_bar\"", "\"light_status_bar\"", "\"accent_color\"", "\"primary_color\"", "\"back_color\""];
  static ref THEME_NAME_PATTERN: Regex = Regex::new(r#""name"\s*:\s*"(.+)""#).unwrap();
}

pub fn detect_and_extract_theme_json(total_text: &str, out_spannables: &mut Vec<Spannable>) {
  if !text_contains_valid_json(total_text) {
    return;
  }

  let json_parts = collect_all_json_parts(total_text);
  if json_parts.is_empty() {
    return;
  }

  for (json_open_bracket_index, json_end_bracket_index) in json_parts {
    if !is_probably_theme_json(&total_text[json_open_bracket_index..json_end_bracket_index]) {
      continue;
    }

    let theme_name = try_extract_theme_name(&total_text[json_open_bracket_index..json_end_bracket_index]);

    let spannable = Spannable {
      start: json_open_bracket_index,
      len: json_end_bracket_index - json_open_bracket_index,
      spannable_data: SpannableData::ThemeJson { theme_name }
    };

    if spannable.is_valid() {
      out_spannables.push(spannable);
    }
  }
}

fn try_extract_theme_name(theme_json: &str) -> String {
  let theme_name_captures_maybe = THEME_NAME_PATTERN.captures(theme_json);
  if theme_name_captures_maybe.is_none() {
    return "Unknown theme name".to_string();
  }

  let theme_name_match_maybe = theme_name_captures_maybe.unwrap().get(1);
  if theme_name_match_maybe.is_none() {
    return "Unknown theme name".to_string();
  }

  return theme_name_match_maybe.unwrap().as_str().to_string();
}

fn is_probably_theme_json(theme_json: &str) -> bool {
  for json_key in THEME_JSON_KEYS.iter() {
    if theme_json.find(json_key).is_none() {
      return false;
    }
  }

  return true;
}

fn collect_all_json_parts(total_text: &str) -> Vec<(usize, usize)> {
  let mut json_open_bracket_index = -1;
  let mut opened_brackets_count = 0;
  let mut result_vec: Vec<(usize, usize)> = Vec::new();

  if total_text.is_empty() {
    return result_vec;
  }

  for (index, char) in total_text.char_indices() {
    if char == '{' as char {
      opened_brackets_count += 1;

      if json_open_bracket_index == -1 {
        json_open_bracket_index = index as i32;
      }

      continue;
    }

    if char == '}' as char {
      opened_brackets_count -= 1;

      if opened_brackets_count < 0 {
        break;
      }

      if opened_brackets_count == 0 {
        result_vec.push((json_open_bracket_index as usize, index + 1));
        json_open_bracket_index = -1;
      }

      continue;
    }
  }

  return result_vec;
}

fn text_contains_valid_json(total_text: &str) -> bool {
  let mut json_open_bracket_index = -1;
  let mut json_close_bracket_index = -1;

  for (index, char) in total_text.char_indices() {
    if char == '{' as char && json_open_bracket_index == -1 {
      json_open_bracket_index = index as i32;
    }

    if char == '}' {
      json_close_bracket_index = index as i32;
    }

    if json_open_bracket_index >= 0 && json_close_bracket_index >= 0 {
      break;
    }
  }

  return json_open_bracket_index >= 0 && json_close_bracket_index >= 0 && json_close_bracket_index > json_open_bracket_index
}

#[test]
fn test_simple_theme_json() {
  let text = "{
\"name\": \"Kuromakaba Light\",
\"is_light_theme\": true,
\"light_nav_bar\": true,
\"light_status_bar\": true,
\"accent_color\": \"#f3630e\",
\"primary_color\": \"#000000\",
\"back_color\": \"#e2e2e2\",
\"post_details_color\": \"#6a6a6a\",
\"post_highlight_quote_color\": \"#f3630e\",
\"post_highlighted_color\": \"#f1f1f1\",
\"post_inline_quote_color\": \"#899918\",
\"post_link_color\": \"#f3630e\",
\"post_name_color\": \"#6a6a6a\",
\"post_quote_color\": \"#f3630e\",
\"post_saved_reply_color\": \"#f1f1f1\",
\"post_spoiler_color\": \"#c6c6c6\",
\"post_spoiler_reveal_text_color\": \"#303030\",
\"post_subject_color\": \"#134b6b\",
\"post_unseen_label_color\": \"#ea8162\",
\"text_color_hint\": \"#6a6a6a\",
\"text_color_primary\": \"#474747\",
\"text_color_secondary\": \"#6a6a6a\",
\"bookmark_counter_has_replies_color\": \"#f3630e\",
\"bookmark_counter_normal_color\": \"#474747\",
\"bookmark_counter_not_watching_color\": \"#6a6a6a\",
\"divider_color\": \"#c6c6c6\",
\"error_color\": \"#ff0000\"
}";

  let json_parts = collect_all_json_parts(text);

  assert_eq!(1, json_parts.len());
  assert_eq!(0, json_parts[0].0);
  assert_eq!(text.len() - 1, json_parts[0].1);
}

#[test]
fn test_complex_nested_json() {
  let text = "{1{2}}{{{}}}{}arar{w{r{wwa}}}";

  let json_parts = collect_all_json_parts(text);
  assert_eq!(4, json_parts.len());

  assert_eq!("{1{2}}", &text[json_parts[0].0..json_parts[0].1]);
  assert_eq!("{{{}}}", &text[json_parts[1].0..json_parts[1].1]);
  assert_eq!("{}", &text[json_parts[2].0..json_parts[2].1]);
  assert_eq!("{w{r{wwa}}}", &text[json_parts[3].0..json_parts[3].1]);
}

#[test]
fn test_invalid_json1() {
  let text = "{";

  let json_parts = collect_all_json_parts(text);
  assert_eq!(0, json_parts.len());
}

#[test]
fn test_invalid_json2() {
  let text = "}";

  let json_parts = collect_all_json_parts(text);
  assert_eq!(0, json_parts.len());
}