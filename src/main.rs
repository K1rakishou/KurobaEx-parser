use serde::{Serialize, Deserialize};
use parser::parser::*;
use std::borrow::Borrow;

mod parser;
mod comment_parser;

mod rules {
    pub mod anchor;
    pub mod line_break;
    pub mod rule_handler;
}

#[derive(Deserialize)]
struct PostRaw {
    com: Option<String>,
}

#[derive(Deserialize)]
struct ThreadRaw {
    posts: Vec<PostRaw>,
}

// https://a.4cdn.org/vg/thread/333444848.json
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let thread_raw = reqwest::get("https://a.4cdn.org/vg/thread/333444848.json")
    //   .await?
    //   .json::<ThreadRaw>()
    //   .await?;

    let postCommentRaw = "<a href=\"#p333520145\" class=\"quotelink\">&gt;&gt;333520145</a><br><a href=\"#p333520391\" class=\"quotelink\">&gt;&gt;333520391</a><br>
Feel free to tell me specifically what I&#039;m wrong about. I&#039;ll take one thing he says: that Tomoya is behaving negatively by &quot;dragging her down.&quot;
But Tomoya knows that. As soon as she becomes the student council president their relationship becomes a political issue with teachers telling her to break up with him,
(not because she has a boyfriend, but because it&#039;s him specifically), so his perception of the relationship is realistic.
She is incapable of achieving her goal while dating him, but she holds their relationship equal with her goal and so cannot end it herself,
which is why he dumps her, so that she&#039;ll be able to achieve it. Stop talking about a game you haven&#039;t even read.
Also the ending in the anime OVA is not quite the same as the game, so having seen that alone doesn&#039;t quality you to have anything to say about this.";
    let thread_raw = ThreadRaw {
        posts: vec![
            PostRaw {
                com: Option::Some(String::from(postCommentRaw))
            }
        ]
    };

    let post_parser = PostParser::new();

    for post_raw in thread_raw.posts {
        if post_raw.com.is_none() {
            continue;
        }

        // let comment_unparsed = String::from(post_raw.com.clone().unwrap());
        // println!("comment_unparsed: {:?}", comment_unparsed);

        let post_comment_parsed = parse_post(&post_raw, &post_parser).post_comment_parsed.unwrap();
        println!("comment: \n{}", post_comment_parsed.comment_text);

        for spannable in post_comment_parsed.spannables.iter() {
            println!("spannable: \n{:?}", spannable);
        }
    }


    Ok(())
}

fn parse_post(post_raw: &PostRaw, post_parser: &PostParser) -> ParsedPost {
    let mut post = ParsedPost::new(Option::None);

    let comment = post_raw.com.as_ref();
    if comment.is_some() {
        post.post_comment_parsed = post_parser.parse_comment(comment.unwrap().as_str());
    }

    return post
}