use crate::{
    app::SITE_CONFIGURATION,
    components::{
        articles::list::{
            articles_list::ArticlesList, articles_pagination::ArticlesPagination,
            articles_title::ArticleTitleBar,
        },
        error_page::ErrorPage,
        progress_bar::stop_progress_bar,
    },
    models::{self, ArticleSearchIndex, SearchCriteria, SearchableArticle},
};
use dioxus::prelude::*;

#[component]
pub fn ArticlesListPage() -> Element {
    let site = SITE_CONFIGURATION
        .get()
        .expect("Site configuration not initialized");
    let mut search_query = use_signal(|| String::new());
    let search_expanded = use_signal(|| false);
    let mut current_page = use_signal(|| 0usize);

    const ARTICLES_PER_PAGE: usize = 10;

    let site_clone = site.clone();
    let articles_index = use_resource(move || {
        let site = site_clone.clone();
        async move {
            let site = site.clone();
            models::ArticleIndex::fetch(&site)
                .await
                .map(|index| index.to_search_index())
        }
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

    let handle_page_change = move |page: usize| {
        current_page.set(page);
    };
    let articles_guard = articles_index.read();
    match articles_guard.as_ref() {
        Some(Ok(search_index)) => {
            // Parse search criteria from query
            let search_criteria = SearchCriteria::parse(&search_query.read());

            // Filter articles using the new search criteria
            let filtered_articles: Vec<&SearchableArticle> =
                search_index.search_with_criteria(&search_criteria);

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
                if search_criteria.is_empty() {
                    "No articles yet!".to_string()
                } else {
                    "No articles found matching your search criteria.".to_string()
                }
            } else {
                "No articles on this page.".to_string()
            };
            stop_progress_bar();

            rsx! {
                div {
                    class: "articles-list-container",

                    ArticleTitleBar {
                        search_query,
                        search_expanded,
                        on_search_change: handle_search_change,
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
