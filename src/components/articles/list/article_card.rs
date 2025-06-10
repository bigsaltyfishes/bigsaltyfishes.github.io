use crate::models::SearchableArticle;
use leptos::attr::global::ClassAttribute;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ArticleCard(article: SearchableArticle) -> impl IntoView {
    view! {
        // List item with bottom border and spacing, removing them for the last item
        <li class="article-card">

            // Header containing title and meta info
            <div class="article-card-header">
                <h2 class="article-card-title">
                    <A
                        href=format!("/articles/{}", article.id)
                        attr:class="article-card-title-link"
                    >
                        {article.article.title.clone()}
                    </A>
                </h2>

                // Meta information (category and tags)
                <div class="article-card-meta">
                    {article
                        .article
                        .category
                        .as_ref()
                        .map(|category| {
                            view! { <span class="article-card-category">{category.clone()}</span> }
                        })}
                    {(!article.article.tags.is_empty())
                        .then(|| {
                            view! {
                                <div class="article-card-tags">
                                    {article
                                        .article
                                        .tags
                                        .iter()
                                        .map(|tag| {
                                            view! {
                                                <span class="article-card-tag">{format!("#{}", tag)}</span>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            }
                        })}
                </div>
            </div>

            // Article description
            <p class="article-card-description">{article.article.description.clone()}</p>
        </li>
    }
}
