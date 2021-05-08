use crate::SpannableData;
use std::collections::HashMap;
use crate::util::color_decoder::color_name_to_color_hex;

lazy_static::lazy_static! {
  static ref STYLE_DECODERS: HashMap<&'static str, fn(&str) -> Option<SpannableData>> = {
    let mut map = HashMap::<_, fn(&str) -> _>::new();

    // font-size:22px;font-size:150%;
    map.insert("font-size", |style_value| {
      return Option::Some(SpannableData::FontSize { size: String::from(style_value) });
    });

    // font-weight:600;font-weight:bold
    map.insert("font-weight", |_| {
      // TODO
      return Option::None;
    });

    // color:red;color:#fd4d32
    map.insert("color", |style_value| {
      if style_value.starts_with("#") {
        return Option::Some(SpannableData::TextForegroundColorRaw { color_hex: style_value.to_string() })
      }

      let color_hex_maybe = color_name_to_color_hex(style_value);

      if let Option::None = color_hex_maybe {
        return Option::None;
      }

      return Option::Some(SpannableData::TextForegroundColorRaw { color_hex: color_hex_maybe.unwrap() });
    });

    return map;
  };
}

pub fn decode_style_spans(style_attr_value: &str) -> Vec<SpannableData> {
  if style_attr_value.is_empty() {
    return Vec::new();
  }

  let style_params: Vec<&str> = style_attr_value.split(';').collect();
  if style_params.is_empty() {
    return Vec::new();
  }

  let mut output_spannables: Vec<SpannableData> = Vec::with_capacity(style_params.len());

  for style_param in style_params {
    if style_param.is_empty() {
      continue;
    }

    let style_param_parts: Vec<&str> = style_param.split(':').collect();
    if style_param_parts.len() != 2 {
      continue;
    }

    let style_param_name = style_param_parts.get(0).unwrap().trim();
    let style_param_value = style_param_parts.get(1).unwrap().trim();
    let decoder_maybe = STYLE_DECODERS.get(style_param_name);

    let decoder = if let Option::None = decoder_maybe {
      continue
    } else {
      decoder_maybe.unwrap()
    };

    let decoded_spannable_data = decoder(style_param_value);

    if let Option::Some { .. } = decoded_spannable_data {
      output_spannables.push(decoded_spannable_data.unwrap());
    }
  }

  return output_spannables;
}

#[test]
fn test_decode_style_spans() {
  assert_eq!(
    &SpannableData::FontSize { size: String::from("22px") },
    decode_style_spans("font-size:22px").first().unwrap()
  );
  assert_eq!(
    &SpannableData::FontSize { size: String::from("150%") },
    decode_style_spans("font-size:150%").first().unwrap()
  );

  assert_eq!(Option::None, decode_style_spans("font-weight:600").first());
  assert_eq!(Option::None, decode_style_spans("font-weight:bold").first());

  assert_eq!(
    &SpannableData::TextForegroundColorRaw { color_hex: String::from("#FF0000") },
    decode_style_spans("color:red").first().unwrap());
  assert_eq!(
    &SpannableData::TextForegroundColorRaw {color_hex: String::from("#FD4D32") },
    decode_style_spans("color:#FD4D32").first().unwrap());
  assert_eq!(
    &SpannableData::TextForegroundColorRaw {color_hex: String::from("#fd4d32") },
    decode_style_spans("color:#fd4d32").first().unwrap());
}