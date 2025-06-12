use katex_wasmbind::KaTeXOptions;
use leptos::prelude::*;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd, TextMergeStream};
use std::fmt::Debug;

use crate::{app::SITE_CONFIGURATION, bindgen, components::footer::Footer};

pub struct MarkdownArticle {
    id: String,
    content: String,
}

impl MarkdownArticle {
    pub fn new(content: String, id: String) -> Self {
        Self { id, content }
    }

    // The render_markdown and try_rewrite_assets_link functions remain the same.
    // They are not directly affected by the CSS framework change.
    fn render_markdown(&self) -> String {
        let mut html_output = String::new();
        let mut in_code_block = false;
        let mut lang = String::new();
        let mut iterator = Vec::new();
        let events = Parser::new_ext(&self.content, Options::all());
        for e in TextMergeStream::new(events) {
            match e {
                Event::Start(Tag::CodeBlock(kind)) => {
                    match kind {
                        CodeBlockKind::Fenced(lang_str) => {
                            lang = lang_str.to_string();
                        }
                        CodeBlockKind::Indented => {
                            lang.clear(); // Clear language marker for indented code blocks without language info
                        }
                    }
                    in_code_block = true;
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                }
                Event::Text(text) => {
                    if in_code_block {
                        let output =
                            format!("<pre>{}</pre>", bindgen::highlight_code(&text, &lang));
                        iterator.push(Event::Html(output.into()));
                    } else {
                        iterator.push(Event::Text(text));
                    }
                }
                Event::DisplayMath(equation) => {
                    let d = KaTeXOptions::display_mode();
                    iterator.push(Event::Html(d.render(&equation).into()));
                }
                Event::InlineMath(equation) => {
                    let i = KaTeXOptions::inline_mode();
                    iterator.push(Event::Html(i.render(&equation).into()));
                }
                Event::Start(Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) => {
                    if let Some(rewritten_url) = self.try_rewrite_assets_link(&dest_url) {
                        // Rewrite asset links to point to the correct assets directory
                        iterator.push(Event::Start(Tag::Link {
                            link_type,
                            dest_url: rewritten_url.into(),
                            title,
                            id,
                        }));
                    } else {
                        // Fallback to original link
                        iterator.push(Event::Start(Tag::Link {
                            link_type,
                            dest_url,
                            title,
                            id,
                        }));
                    }
                }
                Event::Start(Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                }) => {
                    // Handle image links
                    if let Some(rewritten_url) = self.try_rewrite_assets_link(&dest_url) {
                        // Rewrite asset links to point to the correct assets directory
                        iterator.push(Event::Start(Tag::Image {
                            link_type,
                            dest_url: rewritten_url.into(),
                            title,
                            id,
                        }));
                    } else {
                        // Fallback to original image link
                        iterator.push(Event::Start(Tag::Image {
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

        html::push_html(&mut html_output, iterator.into_iter());

        let footer_html = Footer().build().to_html();

        // Wrap content and footer in a container with proper layout
        format!(
            r#"<div class="markdown-body">{}</div>
            {}
        </div>"#,
            html_output, footer_html
        )
    }

    fn try_rewrite_assets_link(&self, link: &str) -> Option<String> {
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
                    site_config.assets.directory, self.id, asset_path
                );
                return Some(assets_url);
            }
        }
        None
    }
}

impl Into<String> for MarkdownArticle {
    fn into(self) -> String {
        self.render_markdown()
    }
}

impl Debug for MarkdownArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MarkdownArticle(id: {}, content: ...)", self.id)
    }
}

pub trait ToHtml: Mountable {
    fn to_html(&self) -> String {
        self.elements()
            .iter()
            .map(|el| el.outer_html())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl<T: Mountable> ToHtml for T {}
