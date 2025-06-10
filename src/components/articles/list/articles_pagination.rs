use leptos::prelude::*;

#[component]
pub fn ArticlesPagination(
    current_page: RwSignal<usize>,
    #[prop(into)] total_pages: Signal<usize>,
    #[prop(into)] total_articles: Signal<usize>,
    on_page_change: impl Fn(usize) + 'static + Copy + Send,
) -> impl IntoView {
    view! {
        {move || {
            let pages = total_pages.get();
            let articles = total_articles.get();
            if pages <= 1 {
                view! { <div class="pagination-container"></div> }.into_any()
            } else {
                view! {
                    // Pagination container
                    <div class="pagination-container">
                        // Page buttons
                        {(0..pages)
                            .map(|page| {
                                let is_active = move || current_page.get() == page;
                                view! {
                                    <button
                                        class=move || {
                                            if is_active() {
                                                "pagination-button pagination-button-active"
                                            } else {
                                                "pagination-button pagination-button-inactive"
                                            }
                                        }
                                        on:click=move |_| {
                                            on_page_change(page);
                                            if let Some(window) = web_sys::window() {
                                                window.scroll_to_with_x_and_y(0.0, 0.0);
                                            }
                                        }
                                    >
                                        {page + 1}
                                    </button>
                                }
                            })
                            .collect::<Vec<_>>()} // Pagination info text
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
