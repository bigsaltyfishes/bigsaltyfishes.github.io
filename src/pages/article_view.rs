use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use katex_wasmbind::KaTeXOptions;
use pulldown_cmark::{Options, TextMergeStream};

use crate::{
    bindgen,
    components::{error_page::ErrorPage, progress_bar::stop_progress_bar},
    models::Article,
    types::site::SiteContext,
};

#[derive(Props, Clone, PartialEq)]
pub struct ArticlePageProps {
    pub id: String,
}

#[component]
pub fn ArticlePage(props: ArticlePageProps) -> Element {
    let site_context = use_context::<SiteContext>();
    let site = &site_context.0;

    // Use use_resource with dependency on props.id to ensure refresh when route changes
    let article_result = use_resource(use_reactive((&props.id,), |(id,)| {
        let id = id.clone();
        async move {
            Article::fetch(&id).await.map(|(meta, body)| {
                // Article exists, return title and content
                (meta.title, body)
            })
        }
    }));

    let article_result = article_result.read();
    match article_result.as_ref() {
        Some(Ok((title, markdown_content))) => {
            // Article exists, render normally
            let title_clone = title.clone();
            let site_name = site.long();
            use_effect(use_reactive((&props.id,), move |(_,)| {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    document.set_title(&format!("{} - {}", title_clone, site_name));
                }
            }));
            let animation_class = use_signal(|| "page-content");
            let animation_class_clone = animation_class;
            use_effect(use_reactive((&props.id,), move |(_,)| {
                let mut anim_class = animation_class_clone;
                spawn(async move {
                    anim_class.set("page-content");
                    TimeoutFuture::new(10).await;
                    anim_class.set("page-content page-enter-active");
                });
            }));
            // Render Markdown
            let html_output = render_markdown(&markdown_content);
            stop_progress_bar();
            rsx! {
                div {
                    class: "{animation_class.read()}", key: "{props.id}",
                    // Article class "article-content" handles content styling and spacing
                    // No longer using component-rendered page title
                    article { class: "article-content",
                        // Markdown content is now responsible for its own H1 title
                        div { class:"markdown-body", dangerous_inner_html: "{html_output}" }
                    }
                }
            }
        }
        Some(Err(_)) => {
            // Article not found, show 404 error page
            rsx! {
                ErrorPage {
                    title: "Article Not Found".to_string(),
                    message: format!("The article with ID '{}' does not exist.", props.id),
                    error_details: None,
                    on_retry: None,
                    error_type: "404".to_string(),
                    show_navigation: true,
                }
            }
        }
        None => {
            // Loading, render nothing
            rsx! {
                div {}
            }
        }
    }
}

fn render_markdown(markdown_content: &str) -> String {
    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut in_link_block = false;
    let mut link_updated = false;
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
                } else if in_link_block && link_updated {
                    iterator.push(pulldown_cmark::Event::Html(format!("{text}</a>").into()));
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
                in_link_block = true;
                link_updated = false; // Reset link_updated for each new link

                if dest_url.starts_with("/") {
                    match link_type {
                        pulldown_cmark::LinkType::Inline | pulldown_cmark::LinkType::Autolink => {
                            // Create SPA navigation link for other internal routes
                            iterator.push(pulldown_cmark::Event::Html(
                                format!("<a href=\"{dest_url}\" data-spa-link>",).into(),
                            ));
                            link_updated = true;
                        }
                        _ => {}
                    }
                }

                if !link_updated {
                    // Fallback to regular link rendering for external links or non-SPA internal links
                    iterator.push(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }));
                }
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Link) => {
                in_link_block = false;
                if !link_updated {
                    iterator.push(pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Link));
                }
                // Reset link_updated for next link
                link_updated = false;
            }
            _ => iterator.push(e),
        }
    }

    pulldown_cmark::html::push_html(&mut html_output, iterator.into_iter());
    html_output
}
