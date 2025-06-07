use dioxus::prelude::*;

use crate::components::progress_bar::stop_progress_bar;

#[component]
pub fn HomePage() -> Element {
    stop_progress_bar();

    let mut animation_class = use_signal(|| "page-content"); // Initial state for animation trigger
    use_effect(move || {
        animation_class.set("page-content page-enter-active");
    });

    rsx! {
        div {
            class: "home-page-wrapper {animation_class.read()}",
            div { class: "home-page-content",
                h1 { "Welcome" }
                p { "Happy Molyuu Everyday🐟." }
                p { "Navigate using the links in the bar." }
            }
        }
    }
}
