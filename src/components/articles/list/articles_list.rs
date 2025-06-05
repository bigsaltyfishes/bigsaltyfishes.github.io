use crate::components::articles::list::article_card::ArticleCard;
use crate::models::SearchableArticle;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArticlesListProps {
    pub articles: Vec<SearchableArticle>,
    pub is_empty_state: bool,
    pub empty_message: String,
}

#[component]
pub fn ArticlesList(props: ArticlesListProps) -> Element {
    let ArticlesListProps {
        articles,
        is_empty_state,
        empty_message,
    } = props;
    if is_empty_state {
        rsx! {
            div { class: "no-articles",
                p { "{empty_message}" }
            }
        }
    } else {
        rsx! {
            ul { class: "articles-ul",
                for article in articles {
                    ArticleCard {
                        key: "{article.id}",
                        article: article.clone()
                    }
                }
            }
        }
    }
}
