use dioxus::prelude::*;

use crate::components::progress_bar::stop_progress_bar;

#[component]
pub fn HomePage() -> Element {
    let site = crate::app::SITE_CONFIGURATION
        .get()
        .expect("Site configuration not initialized");
    
    // Set the document title
    use_effect(move || {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            document.set_title(format!("Home - {}", site.long()).as_str());
        }
    });

    stop_progress_bar();

    let mut animation_class = use_signal(|| "page-content"); // Initial state for animation triggerAdd commentMore actions
    use_effect(move || {
        animation_class.set("page-content animate-fade-in-up");
    });
    rsx! {
        // Home page wrapper: grows to fill space and centers content.
        // Padding and animation are now handled by the parent layout.
        div {
            class: "home-page-container {animation_class.read()}",
            div {
                h1 { class: "home-page-title", "Welcome" }
                p { class: "home-page-text", "Happy Molyuu Everyday🐟." }
                p { class: "home-page-text", "Navigate using the links in the bar." }
            }
        }
    }
}
