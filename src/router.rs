use crate::pages::article_view::ArticlePage;
use crate::pages::articles_list::ArticlesListPage;
use crate::pages::error_pages::NotFoundPage;
use crate::pages::home::HomePage;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(crate::components::layout::AppLayout)]
    #[route("/")]
    HomePage {},
    
    #[route("/articles")]
    ArticlesListPage {},
    
    #[route("/articles/:id")]
    ArticlePage { id: String },

    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
