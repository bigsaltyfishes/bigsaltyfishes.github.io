use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{
    components::{error_page::ErrorPage, progress_bar::stop_progress_bar},
    models,
    router::Route,
    types::site::SiteContext,
};

#[component]
pub fn ArticlesListPage() -> Element {
    let site_context = use_context::<SiteContext>();
    let site = &site_context.0;
    let articles = use_resource(|| async {
        models::ArticleIndex::fetch().await.map(|index| {
            let mut articles: Vec<_> = index.common.into_iter().collect();
            articles.sort_by_key(|article| article.1.title.clone());
            articles
        })
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

    let articles_guard = articles.read();
    match articles_guard.as_ref() {
        Some(Ok(articles)) => {
            // Articles successfully fetched, render them
            stop_progress_bar();
            rsx! {
                div {
                    class: "articles-list-container {animation_class.read()}",
                    h1 { class: "page-title", "Articles" }
                    if articles.is_empty() {
                        p { "No articles yet!" }
                    } else {
                        ul { class: "articles-ul",
                            for article_entry in articles { // Changed variable name for clarity
                                li {
                                    key: "{article_entry.0}",
                                    h2 {
                                        Link {
                                            to: Route::ArticlePage { id: article_entry.0.clone() },
                                            "{article_entry.1.title}" // Use title from ArticleEntry
                                        }
                                    }
                                    p { "{article_entry.1.description}" }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(Err(_)) => {
            // Error fetching articles, show error page
            // Show 404 error page or similar
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
                // Loading state, render noting
                div {}
            }
        }
    }
}
