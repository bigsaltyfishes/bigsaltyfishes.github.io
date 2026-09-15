use gloo_timers::future::TimeoutFuture;
use leptos::{prelude::*, task::spawn_local};
use leptos_meta::{Meta, Stylesheet, Title};
use leptos_router::{components::A, hooks::use_params_map};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::{
    app::SITE_CONFIGURATION,
    components::{error_page::ErrorPage, layout::ProgressContext, progress_bar::stop_progress_bar},
    models::Article,
    utils::{MarkdownArticle, MarkdownHeading},
};

#[component]
pub fn ArticlePage() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params.with(|params| {
            params
                .get("id")
                .map(|value| value.to_string())
                .unwrap_or_default()
        })
    };

    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");
    let reading_progress = use_context::<ProgressContext>()
        .expect("ProgressContext must be provided by AppLayout")
        .reading_progress;
    let article_result = LocalResource::new(move || {
        let current_id = id();
        async move { Article::fetch(&current_id, site_config).await }
    });

    let content_ready = RwSignal::new(false);
    let animation_class = RwSignal::new("page-content".to_string());
    Effect::new(move |_| {
        animation_class.set("page-content".to_string());
        if content_ready.get() {
            ProgressContext::refresh_progress();
            on_cleanup(move || reading_progress.set(None));

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
                result.as_ref().map_or("Loading...".to_string(), |result| {
                    result.as_ref().map_or("Error loading article".to_string(), |(article, _)| {
                        format!("{} - {}", article.title, site_config.long())
                    })
                })
            })
        } />
        <Meta name="description" content=move || {
            article_result.with(|result| {
                result.as_ref().map_or("Loading...".to_string(), |result| {
                    result.as_ref().map_or("Error loading article".to_string(), |(article, _)| {
                        article.description.chars().take(150).collect::<String>()
                    })
                })
            })
        } />
        <Stylesheet href="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css" />
        <Suspense fallback=|| view! { <div class="article-loading" aria-label="Loading article"></div> }>
            {move || {
                article_result.with(|result| match result {
                    Some(Ok((article, markdown_content))) => {
                        let article_id = id();
                        let markdown = MarkdownArticle::new(markdown_content.clone(), article_id.clone());
                        let headings = markdown.headings();
                        let html_output: String = markdown.into();
                        let cover_url = article
                            .cover
                            .as_ref()
                            .map(|cover| site_config.article_asset_url(&article_id, cover));
                        let tags = article.tags.clone();
                        let category = article.category.clone();
                        let title = article.title.clone();
                        let description = article.description.clone();
                        let date = article.date.clone().unwrap_or_default();
                        let article_path = web_sys::window()
                            .and_then(|window| window.location().pathname().ok())
                            .filter(|path| !path.is_empty())
                            .unwrap_or_else(|| format!("/articles/{article_id}"));
                        content_ready.set(true);

                        view! {
                            <div class=move || format!("page-container article-page {}", animation_class.get())>
                                <article class="article-frame">
                                    <header class="article-head">
                                        <div class="article-head-main">
                                            <A href="/articles" attr:class="article-back">
                                                <span class="material-symbols-outlined" aria-hidden="true">"arrow_back"</span>
                                                "Back to articles"
                                            </A>
                                            <span class="kicker">{category.clone().unwrap_or_else(|| "Article".to_string())}</span>
                                            <h1 class="article-title">{title.clone()}</h1>
                                            <p class="article-deck">{description}</p>
                                            <div class="article-meta">
                                                <div class="article-meta-tags">
                                                    {tags
                                                        .iter()
                                                        .map(|tag| view! { <span class="article-meta-tag">{format!("#{tag}")}</span> })
                                                        .collect_view()}
                                                </div>
                                                <span class="article-meta-date">{date}</span>
                                            </div>
                                        </div>
                                    </header>

                                    <div class="article-cover">
                                        {cover_url
                                            .map(|url| {
                                                view! { <img src=url alt=format!("Cover for {title}") /> }.into_any()
                                            })
                                            .unwrap_or_else(|| {
                                                view! {
                                                    <div class="article-cover-fallback" aria-hidden="true">
                                                        <span class="material-symbols-outlined">"waves"</span>
                                                    </div>
                                                }
                                                .into_any()
                                            })}
                                    </div>

                                    <div class="article-layout">
                                        <div class="markdown-container" inner_html=html_output></div>
                                        <ArticleToc
                                            headings=headings.clone()
                                            article_path=article_path
                                        />
                                    </div>

                                    <div class="article-end">
                                        <A href="/articles" attr:class="next-article">
                                            <span>
                                                <span class="next-label">"Continue browsing"</span>
                                                <span class="next-title">"All articles"</span>
                                            </span>
                                            <span class="next-arrow" aria-hidden="true">"→"</span>
                                        </A>
                                    </div>
                                </article>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(error)) => {
                        let current_id = id();
                        view! {
                            <div class="page-container">
                                <ErrorPage
                                    title="Article Not Found".to_string()
                                    message=format!("The article with ID '{current_id}' does not exist.")
                                    error_details=error.clone()
                                    error_type="404".to_string()
                                    show_navigation=true
                                />
                            </div>
                        }
                            .into_any()
                    }
                    None => view! { <div class="article-loading"></div> }.into_any(),
                })
            }}
        </Suspense>
    }
}

#[component]
fn ArticleToc(headings: Vec<MarkdownHeading>, article_path: String) -> impl IntoView {
    let headings = headings
        .into_iter()
        .filter(|heading| heading.level <= 3)
        .collect::<Vec<_>>();
    let heading_ids = headings
        .iter()
        .map(|heading| heading.id.clone())
        .collect::<Vec<_>>();
    let active_heading = RwSignal::new(heading_ids.first().cloned().unwrap_or_default());

    let heading_ids_for_effect = heading_ids.clone();
    Effect::new(move |_| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let heading_ids = heading_ids_for_effect.clone();
        let update = move || {
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let mut current = heading_ids.first().cloned().unwrap_or_default();
            for id in &heading_ids {
                if let Some(element) = document.get_element_by_id(id) {
                    if element.get_bounding_client_rect().top() <= 150.0 {
                        current = id.clone();
                    }
                }
            }
            active_heading.set(current);
        };
        update();

        let callback = Closure::wrap(
            Box::new(move |_event: web_sys::Event| update()) as Box<dyn FnMut(web_sys::Event)>
        );
        let _ =
            window.add_event_listener_with_callback("scroll", callback.as_ref().unchecked_ref());
        callback.forget();
    });

    view! {
        <aside class=if headings.is_empty() { "toc toc-empty" } else { "toc" }>
            <div class="toc-inner">
                <div class="toc-label">"On this page"</div>
                <nav aria-label="Table of contents">
                    {headings
                        .clone()
                        .into_iter()
                        .map(|heading| {
                            let id = heading.id.clone();
                            let href = format!("{}#{}", article_path, heading.id);
                            let level_class = format!("toc-link toc-level-{}", heading.level);
                            view! {
                                <a
                                    href=href
                                    on:click=|event| event.stop_propagation()
                                    class=move || {
                                        if active_heading.get() == id {
                                            format!("{level_class} toc-link-active")
                                        } else {
                                            level_class.clone()
                                        }
                                    }
                                >
                                    {heading.title}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>
            </div>
        </aside>
    }
}
