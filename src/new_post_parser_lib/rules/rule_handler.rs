use crate::{PostRaw, PostParserContext, Element, Spannable, TextPart};
use crate::util::helpers::{SumBy, MapJoin};

pub trait RuleHandler {

  fn pre_handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    out_text_parts: &mut Vec<TextPart>,
    out_spannables: &mut Vec<Spannable>
  ) -> bool;

  fn post_handle(
    &self,
    post_raw: &PostRaw,
    post_parser_context: &PostParserContext,
    element: &Element,
    prev_out_text_parts_index: usize,
    out_text_parts: &mut Vec<TextPart>,
    prev_out_spannables_index: usize,
    out_spannables: &mut Vec<Spannable>
  );

}

pub trait RuleHandlerPostHandleMeta {
  fn get_out_text_parts_diff_text(&self, prev_out_text_parts_index: usize, out_text_parts: &Vec<TextPart>) -> String;
  fn get_out_text_parts_diff_len(&self, prev_out_text_parts_index: usize, out_text_parts: &Vec<TextPart>) -> i32;
  fn get_out_text_parts_new_len(&self, prev_out_text_parts_index: usize, out_text_parts: &Vec<TextPart>) -> i32;
}

impl RuleHandlerPostHandleMeta for dyn RuleHandler {

  fn get_out_text_parts_diff_text(&self, prev_out_text_parts_index: usize, out_text_parts: &Vec<TextPart>) -> String {
    return out_text_parts[prev_out_text_parts_index..]
      .iter()
      .map_join(&|text_part| text_part.text.as_str());
  }

  fn get_out_text_parts_diff_len(&self, prev_out_text_parts_index: usize, out_text_parts: &Vec<TextPart>) -> i32 {
    if prev_out_text_parts_index <= 0 {
      return 0;
    }

    return out_text_parts[0..prev_out_text_parts_index]
      .iter()
      .sum_by(&|string| string.characters_count as i32);
  }

  fn get_out_text_parts_new_len(&self, prev_out_text_parts_index: usize, out_text_parts: &Vec<TextPart>) -> i32 {
    return out_text_parts[prev_out_text_parts_index..]
      .iter()
      .sum_by(&|string| string.characters_count as i32);
  }

}