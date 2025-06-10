use crate::components::layout::AppLayout;
use crate::pages::article_view::ArticlePage;
use crate::pages::articles_list::ArticlesListPage;
use crate::pages::error_pages::NotFoundPage;
use crate::pages::home::HomePage;
use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| {
                view! { <NotFoundPage /> }
            }>
                <ParentRoute path=path!("") view=AppLayout>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("articles") view=ArticlesListPage />
                    <Route path=path!("articles/:id") view=ArticlePage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
