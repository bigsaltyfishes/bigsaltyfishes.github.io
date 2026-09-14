use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

use crate::{
    app::SITE_CONFIGURATION,
    components::{articles::list::ArticleCard, progress_bar::stop_progress_bar},
    models::ArticleIndex,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let site = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");
    let site_name = site.long();
    let author_github = site.author.github.clone();
    let author_email = site.author.email.clone();
    let welcome_title = site.home.welcome_title.clone();
    let welcome_text = site.home.welcome_text.clone();

    let site_for_articles = site.clone();
    let recent_articles = LocalResource::new(move || {
        let site = site_for_articles.clone();
        async move {
            ArticleIndex::fetch(&site)
                .await
                .map(|index| index.to_search_index())
        }
    });

    let animation_class = RwSignal::new("page-content".to_string());
    Effect::new(move |_| {
        animation_class.set("page-content animate-fade-in-up".to_string());
        stop_progress_bar();
    });

    view! {
        <Title text=format!("Home - {site_name}") />
        <div class=move || format!("page-container {}", animation_class.get())>
            <section class="shell home-hero">
                <div class="home-hero-inner">
                    <span class="kicker">"Personal notes · code · life"</span>
                    <h1 class="display-title">
                        {welcome_title}
                        <span class="display-title-subtitle">"Molyuu Blog."</span>
                    </h1>
                    <div class="home-copy">
                        {welcome_text
                            .into_iter()
                            .map(|text| view! { <p>{text}</p> })
                            .collect_view()}
                    </div>
                    <div class="hero-actions">
                        <A href="/articles" attr:class="btn btn-tonal">
                            <span class="material-symbols-outlined" aria-hidden="true">"menu_book"</span>
                            "Browse articles"
                        </A>
                        <A href="/about" attr:class="btn btn-outlined">
                            <span class="material-symbols-outlined" aria-hidden="true">"info"</span>
                            "About this site"
                        </A>
                    </div>
                    <div class="socials" aria-label="Social links">
                        <a
                            class="social-link"
                            href=format!("https://github.com/{author_github}")
                            aria-label="GitHub"
                        >
                            <span class="material-symbols-outlined" aria-hidden="true">"code"</span>
                        </a>
                        <a
                            class="social-link"
                            href=format!("mailto:{author_email}")
                            aria-label="Email"
                        >
                            <span class="material-symbols-outlined" aria-hidden="true">"mail"</span>
                        </a>
                    </div>
                </div>
            </section>

            <div class="shell layered-divider" aria-hidden="true"></div>

            <section class="shell recent">
                <div class="section-head">
                    <div>
                        <span class="kicker">"Latest notes"</span>
                        <h2>"Recent posts"</h2>
                    </div>
                    <A href="/articles" attr:class="section-link">"All articles →"</A>
                </div>
                <Suspense fallback=move || {
                    view! {
                        <div class="article-list-skeleton" aria-label="Loading recent articles">
                            <span></span><span></span><span></span>
                        </div>
                    }
                }>
                    {move || {
                        recent_articles.get().map(|result| match result {
                            Ok(index) => {
                                let articles = index.articles.iter().take(3).cloned().collect::<Vec<_>>();
                                view! {
                                    <ul class="articles-list article-list-home">
                                        {articles
                                            .into_iter()
                                            .map(|article| view! { <ArticleCard article=article /> })
                                            .collect_view()}
                                    </ul>
                                }
                                    .into_any()
                            }
                            Err(_) => view! {
                                <p class="muted article-load-note">"Recent posts are taking a little longer to arrive."</p>
                            }
                                .into_any(),
                        })
                    }}
                </Suspense>
            </section>
        </div>
    }
}
