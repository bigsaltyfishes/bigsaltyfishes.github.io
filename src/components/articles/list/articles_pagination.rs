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
    }

    rsx! {
        div { class: "pagination",
            for page in 0..total_pages {
                button {
                    class: if current_page.read().clone() == page { "page-button active" } else { "page-button" },
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
            div { class: "pagination-info",
                "Page {current_page.read().clone() + 1} of {total_pages} ({total_articles} articles)"
            }
        }
    }
}
