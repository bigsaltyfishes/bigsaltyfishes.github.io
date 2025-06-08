use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArticlesPaginationProps {
    pub current_page: Signal<usize>,
    pub total_pages: usize,
    pub total_articles: usize,
    pub on_page_change: EventHandler<usize>,
}

#[component]
pub fn ArticlesPagination(props: ArticlesPaginationProps) -> Element {
    let ArticlesPaginationProps {
        current_page,
        total_pages,
        total_articles,
        on_page_change,
    } = props;

    if total_pages <= 1 {
        return rsx! { div {} };
    }    rsx! {
        // Pagination container
        div {
            class: "pagination-container",
            
            // Page buttons
            for page in 0..total_pages {
                {
                    let is_active = current_page.read().clone() == page;
                    let button_class = if is_active {
                        "pagination-button pagination-button-active"
                    } else {
                        "pagination-button pagination-button-inactive"
                    };

                    rsx! {
                        button {
                            class: "{button_class}",
                            onclick: move |_| {
                                on_page_change.call(page);
                                // Scroll to top
                                if let Some(window) = web_sys::window() {
                                    window.scroll_to_with_x_and_y(0.0, 0.0);
                                }
                            },
                            "{page + 1}"
                        }
                    }
                }
            }
            
            // Pagination info text
            div {
                class: "pagination-info",
                "Page {current_page.read().clone() + 1} of {total_pages} ({total_articles} articles)"
            }
        }
    }
}
