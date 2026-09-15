use leptos::prelude::*;
use leptos_router::components::A;

use crate::models::SearchableArticle;

fn category_icon(category: &str) -> &'static str {
    match category.to_ascii_lowercase().as_str() {
        "technology" | "tech" => "terminal",
        "general" => "edit_note",
        _ => "auto_stories",
    }
}

#[component]
pub fn ArticleCard(article: SearchableArticle) -> impl IntoView {
    let category = article
        .article
        .category
        .clone()
        .unwrap_or_else(|| "Notes".to_string());
    let date = article.article.date.clone().unwrap_or_default();
    let first_tag = article.article.tags.first().cloned();
    let icon = category_icon(&category);

    view! {
        <li class="article-card article-row">
            <A
                href=format!("/articles/{}", article.id)
                attr:class="article-card-link"
            >
                <div class="article-card-thumb" aria-hidden="true">
                    <span class="material-symbols-outlined">{icon}</span>
                </div>
                <div class="article-card-body">
                    <div class="article-card-meta">
                        <span class="article-card-category">{category}</span>
                        {first_tag.map(|tag| view! { <span class="article-card-tag">{format!("#{tag}")}</span> })}
                    </div>
                    <h2 class="article-card-title">{article.article.title.clone()}</h2>
                    <p class="article-card-description">{article.article.description.clone()}</p>
                    <div class="article-card-extra">
                        <span>{article.article.tags.len()} " tags"</span>
                        <span class="article-card-arrow" aria-hidden="true">"↗"</span>
                    </div>
                </div>
                <time class="article-card-date">{date}</time>
            </A>
        </li>
    }
}

#[component]
pub fn ArticleSearchResult(
    article: SearchableArticle,
    on_select: Callback<String>,
) -> impl IntoView {
    let category = article
        .article
        .category
        .clone()
        .unwrap_or_else(|| "Notes".to_string());
    let metadata = format!("{} · {} tags", category, article.article.tags.len());
    let icon = category_icon(&category);
    let article_path = format!("/articles/{}", article.id);

    view! {
        <A
            href=article_path.clone()
            attr:class="search-result"
            on:click=move |event| {
                if event.button() != 0
                    || event.meta_key()
                    || event.alt_key()
                    || event.ctrl_key()
                    || event.shift_key()
                {
                    return;
                }
                event.prevent_default();
                on_select.run(article_path.clone());
            }
        >
            <div class="search-result-thumb" aria-hidden="true">
                <span class="material-symbols-outlined">{icon}</span>
            </div>
            <div>
                <h2 class="search-result-title">{article.article.title.clone()}</h2>
                <div class="search-result-meta">{metadata}</div>
            </div>
        </A>
    }
}
