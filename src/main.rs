#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{ info, Level };
use chrono;

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
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog here"
        }
        div {
            h1 { "High-five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
        StoryListing {}
    }
}

#[component]
fn StoryListing() -> Element {
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
