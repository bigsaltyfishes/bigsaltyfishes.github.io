use crate::{
    components::{
        articles::list::{
            articles_filters::ArticleFilters, articles_title::ArticleTitleBar,
            articles_list::ArticlesList, articles_pagination::ArticlesPagination,
        },
        error_page::ErrorPage,
        progress_bar::stop_progress_bar,
    },
    models::{self, ArticleSearchIndex, SearchableArticle},
    types::site::SiteContext,
};
use dioxus::prelude::*;

#[component]
pub fn ArticlesListPage() -> Element {
    let site_context = use_context::<SiteContext>();
    let site = &site_context.0; // State for search and filtering
    let mut search_query = use_signal(|| String::new());
    let search_expanded = use_signal(|| false);
    let mut current_page = use_signal(|| 0usize);
    let filter_category = use_signal(|| None::<String>);
    let filter_tag = use_signal(|| None::<String>);

    const ARTICLES_PER_PAGE: usize = 10;

    let articles_index = use_resource(|| async {
        models::ArticleIndex::fetch()
            .await
            .map(|index| index.to_search_index())
    });

    let mut animation_class = use_signal(|| "page-content");
    use_effect(move || {
        animation_class.set("page-content page-enter-active");
    });

    use_effect({
        let site_name = site.long();
        move || {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                document.set_title(format!("Articles - {}", site_name).as_str());
            }
        }
    });

    // Event handlers for child components
    let handle_search_change = move |query: String| {
        search_query.set(query);
        current_page.set(0);
    };

    let handle_filter_change = move |_: ()| {
        current_page.set(0);
    };

    let handle_page_change = move |page: usize| {
        current_page.set(page);
    };

    let articles_guard = articles_index.read();
    match articles_guard.as_ref() {
        Some(Ok(search_index)) => {
            // Filter articles based on search and filters
            let filtered_articles: Vec<&SearchableArticle> =
                if let Some(category) = filter_category.read().as_ref() {
                    search_index.filter_by_category(category)
                } else if let Some(tag) = filter_tag.read().as_ref() {
                    search_index.filter_by_tag(tag)
                } else {
                    search_index.search(&search_query.read())
                };

            let total_articles = filtered_articles.len();
            let total_pages = ArticleSearchIndex::total_pages(total_articles, ARTICLES_PER_PAGE);
            let current_page_articles: Vec<SearchableArticle> = ArticleSearchIndex::paginate(
                &filtered_articles,
                current_page.read().clone(),
                ARTICLES_PER_PAGE,
            )
            .iter()
            .map(|&article| article.clone())
            .collect();

            let empty_message = if filtered_articles.is_empty() {
                if search_query.read().is_empty()
                    && filter_category.read().is_none()
                    && filter_tag.read().is_none()
                {
                    "No articles yet!".to_string()
                } else {
                    "No articles found matching your criteria.".to_string()
                }
            } else {
                "No articles on this page.".to_string()
            };

            stop_progress_bar();

            rsx! {
                div {
                    class: "articles-list-container {animation_class.read()}",

                    ArticleTitleBar {
                        search_query,
                        search_expanded,
                        on_search_change: handle_search_change,
                    }

                    ArticleFilters {
                        search_index: search_index.clone(),
                        filter_category,
                        filter_tag,
                        on_filter_change: handle_filter_change,
                    }

                    ArticlesList {
                        articles: current_page_articles,
                        empty_message,
                    }

                    ArticlesPagination {
                        current_page,
                        total_pages,
                        total_articles,
                        on_page_change: handle_page_change,
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! {
                ErrorPage {
                    title: "Unexpected Error".to_string(),
                    message: "An unexpected error occurred while fetching articles.".to_string(),
                    error_details: None,
                    on_retry: None,
                    error_type: "500".to_string(),
                    show_navigation: true,
                }
            }
        }
        None => {
            rsx! {
                div {}
            }
        }
    }
}
