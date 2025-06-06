use crate::components::articles::list::article_card::ArticleCard;
use crate::models::SearchableArticle;
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

#[derive(Props, Clone, PartialEq)]
pub struct ArticlesListProps {
    pub articles: Vec<SearchableArticle>,
    pub empty_message: String,
}

#[component]
pub fn ArticlesList(props: ArticlesListProps) -> Element {
    let ArticlesListProps {
        articles,
        empty_message,
    } = props;
    let init_signal = use_signal(|| true);
    let show_group = use_signal(|| true);
    let update_msg = use_signal(|| empty_message.clone());
    let update_group = use_signal(|| articles.clone());

    use_effect({
        let reactive_bundle = (articles, empty_message);
        let mut init_signal = init_signal.clone();
        let mut show_group = show_group.clone();
        let mut update_msg = update_msg.clone();
        let mut update_group = update_group.clone();
        use_reactive((&reactive_bundle,), move |((articles, empty_message),)| {
            spawn(async move {
                if *init_signal.read() {
                    // No load animation on initial load
                    init_signal.set(false);
                    return;
                }
                show_group.set(false);
                TimeoutFuture::new(400).await;
                update_msg.set(empty_message);
                show_group.set(true);
                update_group.set(articles);
            });
        })
    });

    let update_group = update_group.read();
    let update_msg = update_msg.read();
    rsx! {
        div {
            class: if *show_group.read() { "article-cards" } else { "article-cards hidden" },
            if update_group.is_empty() {
                div {
                    class: "no-articles",
                    p { "{update_msg}" }
                }
            } else {
                ul {
                    class: "articles-ul",
                    for article in update_group.iter() {
                        ArticleCard {
                            key: "{article.id}",
                            article: article.clone()
                        }
                    }
                }
            }
        }
    }
}
