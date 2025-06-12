use crate::{
    app::SITE_CONFIGURATION,
    components::{
        articles::list::{ArticleTitleBar, ArticlesList, ArticlesPagination},
        error_page::ErrorPage,
        footer::Footer,
        progress_bar::stop_progress_bar,
    },
    models::{self, ArticleSearchIndex, SearchCriteria},
};
use leptos::prelude::*;
use leptos_meta::Title;

const ARTICLES_PER_PAGE: usize = 10;

#[component]
pub fn ArticlesListPage() -> impl IntoView {
    let site = SITE_CONFIGURATION
        .get()
        .expect("Site configuration not initialized");
    let search_query = RwSignal::new(String::new());
    let search_expanded = RwSignal::new(false);
    let current_page = RwSignal::new(0usize);

    let site_clone = site.clone();
    let articles_index = LocalResource::new(move || {
        let site = site_clone.clone();
        async move {
            models::ArticleIndex::fetch(&site)
                .await
                .map(|index| index.to_search_index())
        }
    });
    let animation_class = RwSignal::new("page-content");

    // Page animation - trigger only once when component mounts
    let content_ready = RwSignal::new(false);
    Effect::new(move |_| {
        if content_ready.get() {
            animation_class.set("page-content animate-fade-in-up");
            stop_progress_bar();
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
    view! {
        <Title text=format!("Articles - {}", site.long()) />
        <Suspense fallback=move || {
            view! { <div></div> }
        }>
            {move || {
                articles_index
                    .get()
                    .map(|result| {
                        match result {
                            Ok(search_index) => {
                                content_ready.set(true);

                                view! {
                                    <ArticlesListPageContent
                                        search_index=search_index
                                        search_query=search_query
                                        search_expanded=search_expanded
                                        current_page=current_page
                                        animation_class=animation_class
                                        handle_search_change=handle_search_change
                                        handle_page_change=handle_page_change
                                    />
                                }
                                    .into_any()
                            }
                            Err(_) => {
                                view! {
                                    <ErrorPage
                                        title="Unexpected Error".to_string()
                                        message="An unexpected error occurred while fetching articles."
                                            .to_string()
                                        error_type="500".to_string()
                                        show_navigation=true
                                    />
                                }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
fn ArticlesListPageContent(
    search_index: crate::models::ArticleSearchIndex,
    search_query: RwSignal<String>,
    search_expanded: RwSignal<bool>,
    current_page: RwSignal<usize>,
    animation_class: RwSignal<&'static str>,
    handle_search_change: impl Fn(String) + 'static + Copy + Send,
    handle_page_change: impl Fn(usize) + 'static + Copy + Send,
) -> impl IntoView {
    let filtered_articles = Memo::new(move |_| {
        let criteria = SearchCriteria::parse(&search_query.get());
        search_index
            .search_with_criteria(&criteria)
            .into_iter()
            .map(|article| article.clone())
            .collect::<Vec<_>>()
    });
    let current_page_articles = Memo::new(move |_| {
        let articles = filtered_articles.get();
        let articles_refs: Vec<&_> = articles.iter().collect();
        ArticleSearchIndex::paginate(&articles_refs, current_page.get(), ARTICLES_PER_PAGE)
            .iter()
            .map(|&article| article.clone())
            .collect::<Vec<_>>()
    });

    let empty_message = Memo::new(move |_| {
        let articles = filtered_articles.get();
        let criteria = SearchCriteria::parse(&search_query.get());

        if articles.is_empty() {
            if criteria.is_empty() {
                "No articles yet!".to_string()
            } else {
                "No articles found matching your search criteria.".to_string()
            }
        } else {
            "No articles on this page.".to_string()
        }
    });

    let total_pages = Memo::new(move |_| {
        let articles = filtered_articles.get();
        ArticleSearchIndex::total_pages(articles.len(), ARTICLES_PER_PAGE)
    });

    let total_articles = Memo::new(move |_| filtered_articles.get().len());

    view! {
        // Main container.
        <div class=move || format!("page-container {}", animation_class.get())>
            <div>
                <ArticleTitleBar
                    search_query=search_query
                    search_expanded=search_expanded
                    on_search_change=handle_search_change
                />
                {move || {
                    view! {
                        <ArticlesList articles=current_page_articles empty_message=empty_message />
                    }
                }}
                <ArticlesPagination
                    current_page=current_page
                    total_pages=total_pages
                    total_articles=total_articles
                    on_page_change=handle_page_change
                />
            </div>
            <Footer />
        </div>
    }
}
