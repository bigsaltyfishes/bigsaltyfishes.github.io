use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleSearchBarProps {
    pub search_query: Signal<String>,
    pub search_expanded: Signal<bool>,
    pub on_search_change: EventHandler<String>,
}

#[component]
pub fn ArticleTitleBar(props: ArticleSearchBarProps) -> Element {
    let ArticleSearchBarProps {
        search_query,
        mut search_expanded,
        on_search_change,
    } = props;

    let is_expanded = *search_expanded.read();

    // Auto-focus search input when expanded
    use_effect(move || {
        if *search_expanded.read() {
            spawn_local(async move {
                // A small delay ensures the input is rendered and visible before focusing
                TimeoutFuture::new(50).await;

                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(input) = document.query_selector(".search-input").ok().flatten() {
                        if let Ok(html_input) = input.dyn_into::<web_sys::HtmlInputElement>() {
                            let _ = html_input.focus();
                        }
                    }
                }
            });
        }
    });    rsx! {
        // A container that maintains a minimum height and holds both states.
        div {
            class: "articles-title-container",

            // State 1: Title and Search Icon (Not expanded)
            div {
                class: format!(
                    "articles-title-header {}",
                    if is_expanded { "articles-title-header-hidden" } else { "articles-title-header-visible" }
                ),
                h1 {
                    class: "articles-title",
                    "Articles"
                }
                button {
                    class: "articles-search-button",
                    onclick: move |_| { search_expanded.set(true); },                    span {
                        // Added `translate-y-px` to vertically align the icon.
                        class: "material-symbols-outlined articles-search-icon",
                        "search"
                    }
                }
            }

            // State 2: Full-width search input (Expanded)
            // This is absolutely positioned to overlay the other state.
            div {
                class: format!(
                    "articles-search-input-container {}",
                    if is_expanded { "articles-search-input-visible" } else { "articles-search-input-hidden" }
                ),
                input {
                    class: "search-input articles-search-input",
                    r#type: "text",
                    placeholder: "Search: category:<any> tag:<any> keywords",
                    value: "{search_query.read()}",
                    oninput: move |evt| { on_search_change.call(evt.value()); },
                    onblur: move |_| {
                        if search_query.read().is_empty() {
                            search_expanded.set(false);
                        }
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            search_expanded.set(false);
                        }
                    }
                }
            }
        }
    }
}
