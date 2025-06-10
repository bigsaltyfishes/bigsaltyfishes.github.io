use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::{progress_bar::stop_progress_bar, ErrorPage};

#[component]
pub fn NotFoundPage() -> impl IntoView {
    stop_progress_bar();

    let location = use_location();
    let requested_path = location.pathname.get();

    view! {
        <ErrorPage
            title="Page Not Found".to_string()
            message=format!("Sorry, the page you requested ({}) does not exist.", requested_path)
            error_type="404".to_string()
            show_navigation=true
        />
    }
}

#[component]
pub fn ServerErrorPage() -> impl IntoView {
    stop_progress_bar();

    view! {
        <ErrorPage
            title="Internal Server Error".to_string()
            message="An unexpected error occurred on the server.".to_string()
            error_type="500".to_string()
            show_navigation=true
        />
    }
}

#[component]
pub fn NetworkErrorPage() -> impl IntoView {
    stop_progress_bar();

    view! {
        <ErrorPage
            title="Network Error".to_string()
            message="Unable to connect to the server. Please check your internet connection."
                .to_string()
            error_type="network".to_string()
            show_navigation=true
        />
    }
}
