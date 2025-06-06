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

    // Auto-focus search input when expanded
    use_effect(move || {
        if *search_expanded.read() {
            spawn_local(async move {
                TimeoutFuture::new(10).await;

                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(input) = document.query_selector(".search-input").ok().flatten()
                        {
                            if let Some(html_input) =
                                input.dyn_into::<web_sys::HtmlInputElement>().ok()
                            {
                                let _ = html_input.focus();
                            }
                        }
                    }
                }
            });
        }
    });

    rsx! {
        div {
            class: if *search_expanded.read() { "articles-header search-active" } else { "articles-header" },
            h1 { class: "page-title articles-list", "Articles" }
            div { class: "search-container",
                button {
                    class: "search-toggle",
                    onclick: move |_| {
                        let expanded = *search_expanded.read();
                        search_expanded.set(!expanded);
                    },
                    span { class: "material-symbols-outlined", "search" }
                }
                input {
                    class: if *search_expanded.read() { "search-input expanded" } else { "search-input" },
                    r#type: "text",
                    placeholder: "Search: category:<any> tag:<any> keywords",
                    value: "{search_query.read()}",
                    oninput: move |evt| {
                        on_search_change.call(evt.value());
                    },
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
