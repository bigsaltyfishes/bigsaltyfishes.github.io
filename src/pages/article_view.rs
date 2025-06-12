use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::suspense::Suspense;
use leptos::task::spawn_local;
use leptos_meta::{Meta, Stylesheet, Title};
use leptos_router::hooks::use_params_map;

use crate::{
    app::SITE_CONFIGURATION,
    components::{error_page::ErrorPage, progress_bar::stop_progress_bar},
    models::Article,
    utils::MarkdownArticle,
};

#[component]
pub fn ArticlePage() -> impl IntoView {
    let params = use_params_map();
    let id =
        move || params.with(|params| params.get("id").map(|s| s.to_string()).unwrap_or_default());

    // Get site configuration from global state
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout"); // Use resource with dependency on id to ensure refresh when route changes
    let article_result = LocalResource::new(move || {
        let current_id = id();
        async move {
            Article::fetch(&current_id, &site_config)
                .await
                .map(|(meta, body)| {
                    // Article exists, return title and content
                    (meta.title, meta.description, body)
                })
        }
    });

    let content_ready = RwSignal::new(false);
    let animation_class = RwSignal::new("page-content".to_string());

    Effect::new(move |_| {
        animation_class.set("page-content".to_string());
        if content_ready.get() {
            spawn_local(async move {
                TimeoutFuture::new(10).await;
                animation_class.set("page-content animate-fade-in-up".to_string());
            });
            stop_progress_bar();
        }
    });

    view! {
        <Title text=move || {
            article_result.with(|result| {
                result.as_ref().map_or("Loading...".to_string(), |r| {
                    r.as_ref().map_or("Error loading article".to_string(), |(title, _, _)| {
                        format!("{} - {}", title, site_config.long())
                    })
                })
            })
        } />
        <Meta name="description" content=move || {
            article_result.with(|result| {
                result.as_ref().map_or("Loading...".to_string(), |r| {
                    r.as_ref().map_or("Error loading article".to_string(), |(_, descrition, _)| {
                        descrition.chars().take(150).collect::<String>()
                    })
                })
            })
        } />
        <Stylesheet href="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css" />
        <Suspense fallback=move || {
            view! { <div></div> }
        }>
            {move || {
                article_result
                    .with(|result| {
                        match result {
                            Some(Ok((_, _ , markdown_content))) => {
                                let html_output: String = MarkdownArticle::new(markdown_content.clone(), id()).into();
                                content_ready.set(true);
                                // Article exists, render normally
                                view! {
                                    <div class=move || {
                                        format!("page-container {}", animation_class.get())
                                    }>
                                        <article class="article-content">
                                            <div class="markdown-container" inner_html=html_output></div>
                                        </article>
                                    </div>
                                }
                                    .into_any()
                            }
                            Some(Err(_)) => {
                                let current_id = id();
                                // Article not found, show 404 error page
                                view! {
                                    <div class="page-container">
                                        <ErrorPage
                                            title="Article Not Found".to_string()
                                            message=format!(
                                                "The article with ID '{}' does not exist.",
                                                current_id,
                                            )
                                            error_type="404".to_string()
                                            show_navigation=true
                                        />
                                    </div>
                                }
                                    .into_any()
                            }
                            None => {
                                // Still loading
                                view! { <div></div> }
                                    .into_any()
                            }
                        }
                    })
            }}
        </Suspense>
    }
}
