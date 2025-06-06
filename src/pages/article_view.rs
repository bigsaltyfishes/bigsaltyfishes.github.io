use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use katex_wasmbind::KaTeXOptions;
use pulldown_cmark::{Options, TextMergeStream};

use crate::{
    app::SITE_CONFIGURATION,
    bindgen,
    components::{error_page::ErrorPage, progress_bar::stop_progress_bar},
    models::Article,
};

#[derive(Props, Clone, PartialEq)]
pub struct ArticlePageProps {
    pub id: String,
}

#[component]
pub fn ArticlePage(props: ArticlePageProps) -> Element {
    let site = SITE_CONFIGURATION
        .get()
        .expect("Site configuration not initialized");

    // Use use_resource with dependency on props.id to ensure refresh when route changes
    let article_result = use_resource({
        let site = site.clone();
        use_reactive((&props.id,), move |(id,)| {
            let id = id.clone();
            let site = site.clone();
            async move {
                Article::fetch(&id, &site).await.map(|(meta, body)| {
                    // Article exists, return title and content
                    (meta.title, body)
                })
            }
        })
    });

    let animation_class = use_signal(|| "page-content");

    // Effect for updating page title
    use_effect(use_reactive((&props.id,), move |(_,)| {
        let article_result = article_result.read();
        if let Some(Ok((title, _))) = article_result.as_ref() {
            let title_clone = title.clone();
            let site_name = site.long();
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                document.set_title(&format!("{} - {}", title_clone, site_name));
            }
        }
    }));

    // Effect for animation
    let animation_class_clone = animation_class;
    use_effect(use_reactive((&props.id,), move |(_,)| {
        let mut anim_class = animation_class_clone;
        spawn(async move {
            anim_class.set("page-content");
            TimeoutFuture::new(10).await;
            anim_class.set("page-content page-enter-active");
        });
    }));

    let article_result = article_result.read();
    match article_result.as_ref() {
        Some(Ok((_, markdown_content))) => {
            // Article exists, render normally
            // Render Markdown
            let html_output = render_markdown(&markdown_content, &props.id);

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

fn render_markdown(markdown_content: &str, article_id: &str) -> String {
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

                if let Some(rewritten_url) = try_rewrite_assets_link(&dest_url, article_id) {
                    // Rewrite asset links to point to the correct assets directory
                    iterator.push(pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link {
                        link_type,
                        dest_url: rewritten_url.into(),
                        title,
                        id,
                    }));
                    continue; // Skip to next event to avoid pushing the original link
                }

                if dest_url.starts_with("/") {
                    match link_type {
                        pulldown_cmark::LinkType::Inline => {
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
    let site = SITE_CONFIGURATION
        .get()
        .expect("Site configuration not initialized");
    let assets_re = web_sys::js_sys::RegExp::new(r"\/\$ASSETS\/(.+)", "i");
    let assets_match = assets_re.exec(&link);
    if let Some(m) = assets_match {
        // Replace $ASSETS with the actual assets URL
        let asset_path = m.get(1).as_string();
        if let Some(asset_path) = asset_path {
            // Rewrite the URL to point to the assets directory
            let assets_url = format!(
                "/{}/{}/{}/{}",
                site.assets.directory, site.assets.articles, article_id, asset_path
            );
            return Some(assets_url);
        }
    }
    None
}
