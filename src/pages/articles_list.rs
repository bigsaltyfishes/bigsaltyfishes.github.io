use crate::{
    app::SITE_CONFIGURATION,
    components::{
        articles::list::{ArticleTitleBar, ArticlesList, ArticlesPagination},
        error_page::ErrorPage,
        footer::Footer,
        progress_bar::stop_progress_bar,
    },
    models::{ArticleIndex, ArticleSearchIndex, SearchCriteria},
};
use gloo_timers::future::TimeoutFuture;
use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Title;

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
            ArticleIndex::fetch(&site)
                .await
                .map(|index| index.to_search_index())
        }
    });
    let animation_class = RwSignal::new("page-content");
    let pagination_visible = RwSignal::new(true);

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
        // First scroll to top smoothly, then change page after scroll completes
        spawn_local(async move {
            // Scroll to top
            if let Some(window) = web_sys::window() {
                let options = web_sys::ScrollToOptions::new();
                options.set_top(0.0);
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                window.scroll_to_with_scroll_to_options(&options);
            }

            // Wait for smooth scroll to complete
            TimeoutFuture::new(400).await;

            // Now update the page
            current_page.set(page);
        });
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
                                        pagination_visible=pagination_visible
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
    pagination_visible: RwSignal<bool>,
    handle_search_change: impl Fn(String) + 'static + Copy + Send + Sync,
    handle_page_change: impl Fn(usize) + 'static + Copy + Send + Sync,
) -> impl IntoView {
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration not initialized");
    let articles_per_page = site_config.articles.maximum_number_per_page;
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
        ArticleSearchIndex::paginate(&articles_refs, current_page.get(), articles_per_page)
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
        ArticleSearchIndex::total_pages(articles.len(), articles_per_page)
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
                <ArticlesList
                    articles=current_page_articles
                    empty_message=empty_message
                    pagination_visible=pagination_visible
                />
                <ArticlesPagination
                    current_page=current_page
                    total_pages=total_pages
                    total_articles=total_articles
                    pagination_visible=Signal::from(pagination_visible)
                    on_page_change=handle_page_change
                />
            </div>
            <div class=move || {
                if pagination_visible.get() {
                    "transition-opacity duration-[400ms] opacity-100"
                } else {
                    "transition-opacity duration-[400ms] opacity-0"
                }
            }>
                <Footer />
            </div>
        </div>
    }
}
