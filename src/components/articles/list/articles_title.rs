use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn ArticleTitleBar(
    search_query: RwSignal<String>,
    search_expanded: RwSignal<bool>,
    on_search_change: impl Fn(String) + 'static + Copy,
) -> impl IntoView {
    // Auto-focus search input when expanded
    Effect::new(move |_| {
        if search_expanded.get() {
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
    });

    view! {
        // A container that maintains a minimum height and holds both states.
        <div class="articles-title-container">
            // State 1: Title and Search Icon (Not expanded)
            <div class=move || {
                format!(
                    "articles-title-header {}",
                    if search_expanded.get() {
                        "articles-title-header-hidden"
                    } else {
                        "articles-title-header-visible"
                    },
                )
            }>
                <h1 class="articles-title">"Articles"</h1>
                <button
                    class="articles-search-button"
                    on:click=move |_| {
                        search_expanded.set(true);
                    }
                >
                    <span class="material-symbols-outlined articles-search-icon">"search"</span>
                </button>
            </div>

            // State 2: Full-width search input (Expanded)
            // This is absolutely positioned to overlay the other state.
            <div class=move || {
                format!(
                    "articles-search-input-container {}",
                    if search_expanded.get() {
                        "articles-search-input-visible"
                    } else {
                        "articles-search-input-hidden"
                    },
                )
            }>
                <input
                    class="search-input articles-search-input"
                    type="text"
                    placeholder="Search: category:<any> tag:<any> keywords"
                    prop:value=move || search_query.get()
                    on:input=move |evt| {
                        let value = event_target_value(&evt);
                        on_search_change(value);
                    }
                    on:blur=move |_| {
                        if search_query.get().is_empty() {
                            search_expanded.set(false);
                        }
                    }
                    on:keydown=move |evt| {
                        if evt.key() == "Escape" {
                            search_expanded.set(false);
                        }
                    }
                />
            </div>
        </div>
    }
}
