use dioxus::prelude::*;
use crate::components::story_list::{ StoryListing, StoryItem };

#[component]
pub fn Stories() -> Element {
    rsx! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "Hello".to_string(),
                url: None,
                text: None,
                by: "Me".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                children: vec![],
                r#type: "".to_string(),
            },
        }
    }
}
