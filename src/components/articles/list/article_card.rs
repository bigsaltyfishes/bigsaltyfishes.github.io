use crate::{models::SearchableArticle, router::Route};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

#[derive(Props, Clone, PartialEq)]
pub struct ArticleCardProps {
    pub article: SearchableArticle,
}

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    let ArticleCardProps { article } = props;

    rsx! {
        li {
            key: "{article.id}",
            div { class: "article-header",
                h2 {
                    Link {
                        to: Route::ArticlePage { id: article.id.clone() },
                        "{article.article.title}"
                    }
                }
                div { class: "article-meta",
                    if let Some(ref category) = article.article.category {
                        span { class: "category-pill", "{category}" }
                    }
                    if !article.article.tags.is_empty() {
                        div { class: "tags",
                            for tag in &article.article.tags {
                                span { class: "tag-pill", "#{tag}" }
                            }
                        }
                    }
                }
            }
            p { class: "article-description", "{article.article.description}" }
        }
    }
}
