use std::collections::HashMap;

lazy_static! {
  static ref COLOR_MAP: HashMap<&'static str, &'static str> = {
    let mut map = HashMap::new();

    map.insert("aliceblue", "#F0F8FF");
    map.insert("antiquewhite", "#FAEBD7");
    map.insert("aqua", "#00FFFF");
    map.insert("aquamarine", "#7FFFD4");
    map.insert("azure", "#F0FFFF");
    map.insert("beige", "#F5F5DC");
    map.insert("bisque", "#FFE4C4");
    map.insert("black", "#000000");
    map.insert("blanchedalmond", "#FFEBCD");
    map.insert("blue", "#0000FF");
    map.insert("blueviolet", "#8A2BE2");
    map.insert("brown", "#A52A2A");
    map.insert("burlywood", "#DEB887");
    map.insert("cadetblue", "#5F9EA0");
    map.insert("chartreuse", "#7FFF00");
    map.insert("chocolate", "#D2691E");
    map.insert("coral", "#FF7F50");
    map.insert("cornflowerblue", "#6495ED");
    map.insert("cornsilk", "#FFF8DC");
    map.insert("crimson", "#DC143C");
    map.insert("cyan", "#00FFFF");
    map.insert("darkblue", "#00008B");
    map.insert("darkcyan", "#008B8B");
    map.insert("darkgoldenrod", "#B8860B");
    map.insert("darkgray", "#A9A9A9");
    map.insert("darkgrey", "#A9A9A9");
    map.insert("darkgreen", "#006400");
    map.insert("darkkhaki", "#BDB76B");
    map.insert("darkmagenta", "#8B008B");
    map.insert("darkolivegreen", "#556B2F");
    map.insert("darkorange", "#FF8C00");
    map.insert("darkorchid", "#9932CC");
    map.insert("darkred", "#8B0000");
    map.insert("darksalmon", "#E9967A");
    map.insert("darkseagreen", "#8FBC8F");
    map.insert("darkslateblue", "#483D8B");
    map.insert("darkslategray", "#2F4F4F");
    map.insert("darkslategrey", "#2F4F4F");
    map.insert("darkturquoise", "#00CED1");
    map.insert("darkviolet", "#9400D3");
    map.insert("deeppink", "#FF1493");
    map.insert("deepskyblue", "#00BFFF");
    map.insert("dimgray", "#696969");
    map.insert("dimgrey", "#696969");
    map.insert("dodgerblue", "#1E90FF");
    map.insert("firebrick", "#B22222");
    map.insert("floralwhite", "#FFFAF0");
    map.insert("forestgreen", "#228B22");
    map.insert("fuchsia", "#FF00FF");
    map.insert("gainsboro", "#DCDCDC");
    map.insert("ghostwhite", "#F8F8FF");
    map.insert("gold", "#FFD700");
    map.insert("goldenrod", "#DAA520");
    map.insert("gray", "#808080");
    map.insert("grey", "#808080");
    map.insert("green", "#008000");
    map.insert("greenyellow", "#ADFF2F");
    map.insert("honeydew", "#F0FFF0");
    map.insert("hotpink", "#FF69B4");
    map.insert("indianred", "#CD5C5C");
    map.insert("indigo", "#4B0082");
    map.insert("ivory", "#FFFFF0");
    map.insert("khaki", "#F0E68C");
    map.insert("lavender", "#E6E6FA");
    map.insert("lavenderblush", "#FFF0F5");
    map.insert("lawngreen", "#7CFC00");
    map.insert("lemonchiffon", "#FFFACD");
    map.insert("lightblue", "#ADD8E6");
    map.insert("lightcoral", "#F08080");
    map.insert("lightcyan", "#E0FFFF");
    map.insert("lightgoldenrodyellow", "#FAFAD2");
    map.insert("lightgray", "#D3D3D3");
    map.insert("lightgrey", "#D3D3D3");
    map.insert("lightgreen", "#90EE90");
    map.insert("lightpink", "#FFB6C1");
    map.insert("lightsalmon", "#FFA07A");
    map.insert("lightseagreen", "#20B2AA");
    map.insert("lightskyblue", "#87CEFA");
    map.insert("lightslategray", "#778899");
    map.insert("lightslategrey", "#778899");
    map.insert("lightsteelblue", "#B0C4DE");
    map.insert("lightyellow", "#FFFFE0");
    map.insert("lime", "#00FF00");
    map.insert("limegreen", "#32CD32");
    map.insert("linen", "#FAF0E6");
    map.insert("magenta", "#FF00FF");
    map.insert("maroon", "#800000");
    map.insert("mediumaquamarine", "#66CDAA");
    map.insert("mediumblue", "#0000CD");
    map.insert("mediumorchid", "#BA55D3");
    map.insert("mediumpurple", "#9370DB");
    map.insert("mediumseagreen", "#3CB371");
    map.insert("mediumslateblue", "#7B68EE");
    map.insert("mediumspringgreen", "#00FA9A");
    map.insert("mediumturquoise", "#48D1CC");
    map.insert("mediumvioletred", "#C71585");
    map.insert("midnightblue", "#191970");
    map.insert("mintcream", "#F5FFFA");
    map.insert("mistyrose", "#FFE4E1");
    map.insert("moccasin", "#FFE4B5");
    map.insert("navajowhite", "#FFDEAD");
    map.insert("navy", "#000080");
    map.insert("oldlace", "#FDF5E6");
    map.insert("olive", "#808000");
    map.insert("olivedrab", "#6B8E23");
    map.insert("orange", "#FFA500");
    map.insert("orangered", "#FF4500");
    map.insert("orchid", "#DA70D6");
    map.insert("palegoldenrod", "#EEE8AA");
    map.insert("palegreen", "#98FB98");
    map.insert("paleturquoise", "#AFEEEE");
    map.insert("palevioletred", "#DB7093");
    map.insert("papayawhip", "#FFEFD5");
    map.insert("peachpuff", "#FFDAB9");
    map.insert("peru", "#CD853F");
    map.insert("pink", "#FFC0CB");
    map.insert("plum", "#DDA0DD");
    map.insert("powderblue", "#B0E0E6");
    map.insert("purple", "#800080");
    map.insert("rebeccapurple", "#663399");
    map.insert("red", "#FF0000");
    map.insert("rosybrown", "#BC8F8F");
    map.insert("royalblue", "#4169E1");
    map.insert("saddlebrown", "#8B4513");
    map.insert("salmon", "#FA8072");
    map.insert("sandybrown", "#F4A460");
    map.insert("seagreen", "#2E8B57");
    map.insert("seashell", "#FFF5EE");
    map.insert("sienna", "#A0522D");
    map.insert("silver", "#C0C0C0");
    map.insert("skyblue", "#87CEEB");
    map.insert("slateblue", "#6A5ACD");
    map.insert("slategray", "#708090");
    map.insert("slategrey", "#708090");
    map.insert("snow", "#FFFAFA");
    map.insert("springgreen", "#00FF7F");
    map.insert("steelblue", "#4682B4");
    map.insert("tan", "#D2B48C");
    map.insert("teal", "#008080");
    map.insert("thistle", "#D8BFD8");
    map.insert("tomato", "#FF6347");
    map.insert("turquoise", "#40E0D0");
    map.insert("violet", "#EE82EE");
    map.insert("wheat", "#F5DEB3");
    map.insert("white", "#FFFFFF");
    map.insert("whitesmoke", "#F5F5F5");
    map.insert("yellow", "#FFFF00");
    map.insert("yellowgreen", "#9ACD32");

    return map;
  };
}

pub fn color_name_to_color_hex(color_name: &str) -> Option<String> {
  return COLOR_MAP.get(color_name.to_lowercase().as_str())
    .map(|color_hex_str| color_hex_str.to_string());
}

pub fn is_color_hex(color_hex_maybe: &str) -> bool {
  if color_hex_maybe.is_empty() {
    return false;
  }

  let mut has_number_sign = false;

  for character in color_hex_maybe.chars() {
    if character == '#' {
      has_number_sign = true;
      continue;
    }

    if character.is_digit(16) {
      continue;
    }

    return false;
  }

  if !has_number_sign {
    return false;
  }

  return true;
}

#[test]
fn test_get_color() {
  assert_eq!("#D2B48C", color_name_to_color_hex("tan").unwrap());
  assert_eq!("#006400", color_name_to_color_hex("darKgreeN").unwrap());
  assert!(color_name_to_color_hex("test").is_none());
}

#[test]
fn test_is_color_hex() {
  assert!(is_color_hex("#F5DEB3"));
  assert!(is_color_hex("#f5dEB3"));
  assert!(!is_color_hex("f5dEB3"));
  assert!(!is_color_hex("f5dEB3Z"));
  assert!(!is_color_hex(""));
}