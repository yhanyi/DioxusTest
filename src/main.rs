#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{ info, Level };

pub mod components;
use components::story_list::{ StoryItem, StoryListing };
use components::stories::Stories;
use components::preview::{ PreviewState, Preview };

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting app");
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%", font_family: "Arial, sans-serif",
            div { width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}

#[component]
fn Home() -> Element {
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

// use components::high_five::HighFiveCounter;

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

// fn App() -> Element {
//     rsx! {
//         Router::<Route> {}
//     }
// }

// #[component]
// fn Home() -> Element {
//     let count = use_signal(|| 0);
//     rsx! {
//         Link {
//             to: Route::Blog {
//                 id: count()
//             },
//             "Go to blog number {count} here"
//         }
//         HighFiveCounter {count}
//     }
// }
