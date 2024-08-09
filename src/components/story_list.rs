use dioxus::prelude::*;
use chrono;

#[component]
pub fn StoryListing() -> Element {
    let title = "title";
    let by = "author";
    let score = 0;
    let time = chrono::Utc::now();
    let comments = "comments";

    rsx! {
        div {padding: "0.5rem", position:"relative",
              h1{"{title} by {by} ({score}) {time} {comments}"}
            }
    }
}
