use dioxus::prelude::*;

use crate::components::{error_page::ErrorPage, progress_bar::stop_progress_bar};

#[derive(Props, Clone, PartialEq)]
pub struct NotFoundPageProps {
    route: Vec<String>,
}

#[component]
pub fn NotFoundPage(props: NotFoundPageProps) -> Element {
    stop_progress_bar(); // Stop the progress bar when 404 page loads

    let requested_path = if props.route.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", props.route.join("/"))
    };

    rsx! {
        ErrorPage {
            title: "Page Not Found".to_string(),
            message: format!("Sorry, the page you requested ({}) does not exist.", requested_path),
            error_details: None,
            on_retry: None,
            error_type: "404".to_string(),
            show_navigation: true,
        }
    }
}

#[component]
pub fn ServerErrorPage() -> Element {
    stop_progress_bar(); // Stop the progress bar when server error page loads

    rsx! {
        ErrorPage {
            title: "Internal Server Error".to_string(),
            message: "An unexpected error occurred on the server.".to_string(),
            error_details: None,
            on_retry: None,
            error_type: "500".to_string(),
            show_navigation: true,
        }
    }
}

#[component]
pub fn NetworkErrorPage() -> Element {
    stop_progress_bar(); // Stop the progress bar when network error page loads

    rsx! {
        ErrorPage {
            title: "Network Error".to_string(),
            message: "Unable to connect to the server. Please check your internet connection.".to_string(),
            error_details: None,
            on_retry: None,
            error_type: "network".to_string(),
            show_navigation: true,
        }
    }
}
