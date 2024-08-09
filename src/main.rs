#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{ info, Level };

pub mod components;
use components::high_five::HighFiveCounter;
use components::story_list::StoryListing;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")] Home {},
    #[route("/blog/:id")] Blog {
        id: i32,
    },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let count = use_signal(|| 0);
    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog number {count} here"
        }
        HighFiveCounter {count}
        StoryListing {}
    }
}
