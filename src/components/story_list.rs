use dioxus::prelude::*;
use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };
use crate::components::preview::PreviewState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub children: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub children: Vec<i64>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let mut preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem { title, url, by, score, time, children, .. } = &*story.read();
    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });
    let comments = format!("{} {}", children.len(), if children.len() == 1 {
        " comment"
    } else {
        " comments"
    });
    let time = time.format("%D %l:%M %p");
    rsx! {
        div { padding: "0.5rem", position: "relative", onmouseenter: move |_| {},
            div { font_size: "1.5rem",
                a { href: url, onfocus: move |_event| {
                  *preview_state.write() = PreviewState::Loaded(StoryPageData {
                    item: story(),
                    comments: vec![],
                  });
                }, "{title}" }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div { display: "flex", flex_direction: "row", color: "gray",
                div { "{score}" }
                div { padding_left: "0.5rem", "by {by}" }
                div { padding_left: "0.5rem", "{time}" }
                div { padding_left: "0.5rem", "{comments}" }
            }
        }
    }
}
