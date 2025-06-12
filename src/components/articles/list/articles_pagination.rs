use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn ArticlesPagination(
    current_page: RwSignal<usize>,
    #[prop(into)] total_pages: Signal<usize>,
    #[prop(into)] total_articles: Signal<usize>,
    #[prop(into, optional)] pagination_visible: Option<Signal<bool>>,
    on_page_change: impl Fn(usize) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let dropdown_open = RwSignal::new(false);
    let dropdown_closing = RwSignal::new(false);

    // Handle dropdown close with animation
    let close_dropdown = move || {
        if dropdown_open.get() {
            dropdown_closing.set(true);
            spawn_local(async move {
                TimeoutFuture::new(200).await; // Wait for hide animation
                dropdown_open.set(false);
                dropdown_closing.set(false);
            });
        }
    };

    // Handle dropdown toggle
    let toggle_dropdown = move || {
        if dropdown_open.get() {
            close_dropdown();
        } else {
            dropdown_open.set(true);
        }
    };

    view! {
        {move || {
            let pages = total_pages.get();
            let articles = total_articles.get();
            let current = current_page.get();
            if pages <= 1 {

                view! { <div class="pagination-container"></div> }
                    .into_any()
            } else {
                let visible_pages = if pages <= 5 {
                    (0..pages).collect::<Vec<_>>()
                } else if current <= 4 {
                    (0..5).collect::<Vec<_>>()
                } else {
                    let start = if current >= 4 { current - 4 } else { 0 };
                    let end = (start + 5).min(pages);
                    (start..end).collect::<Vec<_>>()
                };
                let has_prev = current > 0;
                let has_next = current < pages - 1;
                let last_visible_page = *visible_pages.last().unwrap_or(&0);
                let show_ellipsis = pages > 5 && last_visible_page < pages - 1;
                // Calculate which pages to show
                // Show all pages if 5 or fewer
                // Show first 5 pages if current page is in the first 5
                // Show current page at the end with 4 pages before it (N-4, N-3, N-2, N-1, N)
                view! {
                    <div class=move || {
                        let visibility_class = if let Some(visible_signal) = pagination_visible {
                            if visible_signal.get() {
                                "pagination-visible"
                            } else {
                                "pagination-hidden"
                            }
                        } else {
                            "pagination-visible"
                        };
                        format!("pagination-container {}", visibility_class)
                    }>
                        // Previous button
                        {has_prev
                            .then(|| {
                                view! {
                                    <button
                                        class="pagination-button pagination-button-inactive"
                                        on:click=move |_| {
                                            on_page_change(current - 1);
                                        }
                                    >
                                        "<"
                                    </button>
                                }
                            })} // Page number buttons
                        {visible_pages
                            .clone()
                            .into_iter()
                            .map(|page| {
                                let is_active = current == page;
                                view! {
                                    <button
                                        class=move || {
                                            if is_active {
                                                "pagination-button pagination-button-active"
                                            } else {
                                                "pagination-button pagination-button-inactive"
                                            }
                                        }
                                        on:click=move |_| {
                                            on_page_change(page);
                                        }
                                    >
                                        {page + 1}
                                    </button>
                                }
                            })
                            .collect_view()} // Ellipsis and dropdown
                        {show_ellipsis
                            .then(|| {
                                view! {
                                    <div class="pagination-dropdown-container">
                                        <button
                                            class="pagination-button pagination-button-inactive"
                                            on:click=move |_| {
                                                toggle_dropdown();
                                            }
                                        >
                                            "..."
                                        </button>
                                        <Show when=move || dropdown_open.get()>
                                            <div
                                                class="pagination-dropdown-overlay"
                                                on:click=move |_| close_dropdown()
                                            ></div>
                                            <div class="pagination-dropdown">
                                                <div
                                                    class=move || {
                                                        if dropdown_closing.get() {
                                                            "pagination-dropdown-content hiding"
                                                        } else {
                                                            "pagination-dropdown-content"
                                                        }
                                                    }
                                                    on:click=move |e| {
                                                        e.stop_propagation();
                                                    }
                                                >
                                                    {
                                                        let last_visible_page = *visible_pages.last().unwrap_or(&0);
                                                        ((last_visible_page + 1)..pages)
                                                            .map(|page| {
                                                                view! {
                                                                    <button
                                                                        class="pagination-dropdown-item"
                                                                        on:click=move |_| {
                                                                            on_page_change(page);
                                                                            close_dropdown();
                                                                        }
                                                                    >
                                                                        {page + 1}
                                                                    </button>
                                                                }
                                                            })
                                                            .collect_view()
                                                    }
                                                </div>
                                            </div>
                                        </Show>
                                    </div>
                                }
                            })} // Next button
                        {has_next
                            .then(|| {
                                view! {
                                    <button
                                        class="pagination-button pagination-button-inactive"
                                        on:click=move |_| {
                                            on_page_change(current + 1);
                                        }
                                    >
                                        ">"
                                    </button>
                                }
                            })} // Pagination info text
                        <div class="pagination-info">
                            {move || {
                                format!(
                                    "Page {} of {} ({} articles)",
                                    current_page.get() + 1,
                                    pages,
                                    articles,
                                )
                            }}
                        </div>
                    </div>
                }
                    .into_any()
            }
        }}
    }
}
