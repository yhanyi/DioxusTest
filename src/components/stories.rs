use dioxus::prelude::*;
use crate::components::story_list::StoryListing;
use crate::components::api::get_stories;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_stories(10));
    match &*stories.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
              div {
                for story in list {
                  StoryListing {story: story.clone()}
                }
              }
            }
        }
        Some(Err(err)) => {
            rsx! {
                "An error occurred while fetching stories {err}"
            }
        }
        None => {
            rsx! {
                "Loading..."
            }
        }
    }

    // rsx! {
    //     StoryListing {
    //         story: StoryItem {
    //             id: 0,
    //             title: "Hello".to_string(),
    //             url: None,
    //             text: None,
    //             by: "Me".to_string(),
    //             score: 0,
    //             descendants: 0,
    //             time: chrono::Utc::now(),
    //             children: vec![],
    //             r#type: "".to_string(),
    //         },
    //     }
    // }
}
