use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use once_cell::sync::OnceCell;

use crate::router::AppRouter;
use crate::types::site::Site;

pub static SITE_CONFIGURATION: OnceCell<Site> = OnceCell::new();

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! { <AppRouter /> }
}
