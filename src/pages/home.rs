use dioxus::prelude::*;

use crate::components::progress_bar::stop_progress_bar;

#[component]
pub fn HomePage() -> Element {
    stop_progress_bar();

    rsx! {
        div {
            class: "home-page-wrapper",
            div { class: "home-page-content",
                h1 { "Welcome" }
                p { "Happy Molyuu Everyday🐟." }
                p { "Navigate using the links in the bar." }
            }
        }
    }
}
