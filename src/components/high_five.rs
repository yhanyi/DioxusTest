use dioxus::prelude::*;

#[component]
pub fn HighFiveCounter(mut count: Signal<i32>) -> Element {
    rsx! {
        div {
            h1 { "High-five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
