use gloo_timers::future::TimeoutFuture;
use katex_wasmbind::KaTeXOptions;
use leptos::prelude::*;
use leptos::suspense::Suspense;
use leptos::task::spawn_local;
use leptos_meta::{Meta, Title, Stylesheet};
use leptos_router::hooks::use_params_map;
use pulldown_cmark::{Options, TextMergeStream};

use crate::{
    app::SITE_CONFIGURATION,
    bindgen,
    components::{error_page::ErrorPage, progress_bar::stop_progress_bar},
    models::Article,
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

    let animation_class = RwSignal::new("page-content".to_string());

    Effect::new(move |_| {
        let _current_id = id(); // Track changes to id
        spawn_local(async move {
            animation_class.set("page-content".to_string());
            TimeoutFuture::new(10).await;
            animation_class.set("page-content animate-fade-in-up".to_string());
        });
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
                                let html_output = render_markdown(markdown_content, &id());
                                stop_progress_bar();
                                
                                // Article exists, render normally
                                view! {
                                    <div class=move || {
                                        format!("article-container {}", animation_class.get())
                                    }>
                                        <article class="article-content">
                                            <div class="markdown-body" inner_html=html_output></div>
                                        </article>
                                    </div>
                                }
                                    .into_any()
                            }
                            Some(Err(_)) => {
                                let current_id = id();
                                // Article not found, show 404 error page
                                view! {
                                    <div class="article-container">
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

// The render_markdown and try_rewrite_assets_link functions remain the same.
// They are not directly affected by the CSS framework change.
fn render_markdown(markdown_content: &str, article_id: &str) -> String {
    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut lang = String::new();
    let mut iterator = Vec::new();
    let events = pulldown_cmark::Parser::new_ext(markdown_content, Options::all());
    for e in TextMergeStream::new(events) {
        match e {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(kind)) => {
                match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang_str) => {
                        lang = lang_str.to_string();
                    }
                    pulldown_cmark::CodeBlockKind::Indented => {
                        lang.clear(); // Clear language marker for indented code blocks without language info
                    }
                }
                in_code_block = true;
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                in_code_block = false;
            }
            pulldown_cmark::Event::Text(text) => {
                if in_code_block {
                    let output = format!("<pre>{}</pre>", bindgen::highlight_code(&text, &lang));
                    iterator.push(pulldown_cmark::Event::Html(output.into()));
                } else {
                    iterator.push(pulldown_cmark::Event::Text(text));
                }
            }
            pulldown_cmark::Event::DisplayMath(equation) => {
                let d = KaTeXOptions::display_mode();
                iterator.push(pulldown_cmark::Event::Html(d.render(&equation).into()));
            }
            pulldown_cmark::Event::InlineMath(equation) => {
                let i = KaTeXOptions::inline_mode();
                iterator.push(pulldown_cmark::Event::Html(i.render(&equation).into()));
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                if let Some(rewritten_url) = try_rewrite_assets_link(&dest_url, article_id) {
                    // Rewrite asset links to point to the correct assets directory
                    iterator.push(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                        link_type,
                        dest_url: rewritten_url.into(),
                        title,
                        id,
                    }));
                } else {
                    // Fallback to original link
                    iterator.push(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }));
                }
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                // Handle image links
                if let Some(rewritten_url) = try_rewrite_assets_link(&dest_url, article_id) {
                    // Rewrite asset links to point to the correct assets directory
                    iterator.push(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image {
                        link_type,
                        dest_url: rewritten_url.into(),
                        title,
                        id,
                    }));
                } else {
                    // Fallback to original image link
                    iterator.push(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }));
                }
            }
            _ => iterator.push(e),
        }
    }

    pulldown_cmark::html::push_html(&mut html_output, iterator.into_iter());
    html_output
}

fn try_rewrite_assets_link(link: &str, article_id: &str) -> Option<String> {
    // For now, use a hardcoded site config that matches the expected structure
    // In a real implementation, this would use the site context
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");
    let assets_re = web_sys::js_sys::RegExp::new(r"\/\$ASSETS\/(.+)", "i");
    let assets_match = assets_re.exec(&link);
    if let Some(m) = assets_match {
        // Replace $ASSETS with the actual assets URL
        let asset_path = m.get(1).as_string();
        if let Some(asset_path) = asset_path {
            // Rewrite the URL to point to the assets directory
            let assets_url = format!(
                "/{}/articles/{}/{}",
                site_config.assets.directory, article_id, asset_path
            );
            return Some(assets_url);
        }
    }
    None
}
