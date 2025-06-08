use crate::{models::SearchableArticle, router::Route};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub article: SearchableArticle,
}

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    let ArticleCardProps { article } = props;    rsx! {
        // List item with bottom border and spacing, removing them for the last item
        li {
            key: "{article.id}",
            class: "article-card",
            
            // Header containing title and meta info
            div {
                class: "article-card-header",
                h2 {
                    class: "article-card-title",
                    Link {
                        to: Route::ArticlePage { id: article.id.clone() },
                        class: "article-card-title-link",
                        "{article.article.title}"
                    }
                }
                
                // Meta information (category and tags)
                div {
                    class: "article-card-meta",
                    if let Some(ref category) = article.article.category {
                        span {
                            class: "article-card-category",
                            "{category}"
                        }
                    }
                    if !article.article.tags.is_empty() {
                        div {
                            class: "article-card-tags",
                            for tag in &article.article.tags {
                                span {
                                    class: "article-card-tag",
                                    "#{tag}"
                                }
                            }
                        }
                    }
                }
            }
            
            // Article description
            p {
                class: "article-card-description",
                "{article.article.description}"
            }
        }
    }
}
