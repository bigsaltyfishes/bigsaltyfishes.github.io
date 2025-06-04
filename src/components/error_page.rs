use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{components::progress_bar::stop_progress_bar, router::Route};

#[derive(Props, Clone, PartialEq)]
pub struct ErrorPageProps {
    pub title: String,
    pub message: String,
    pub error_details: Option<String>,
    pub on_retry: Option<EventHandler<()>>,
    #[props(default = "error".to_string())]
    pub error_type: String,
    #[props(default = false)]
    pub show_navigation: bool,
}

#[component]
pub fn ErrorPage(props: ErrorPageProps) -> Element {
    stop_progress_bar();

    let icon = match props.error_type.as_str() {
        "404" => "unknown_document",
        "500" => "error",
        "network" => "wifi_off",
        _ => "error",
    };

    // Add "error-" prefix to numeric error types to comply with CSS naming conventions
    let css_class = if props
        .error_type
        .chars()
        .next()
        .unwrap_or('a')
        .is_ascii_digit()
    {
        format!("error-{}", props.error_type)
    } else {
        format!("error-{}", props.error_type)
    };

    rsx! {
        div {
            class: "loading-container error",
            div {
                class: "loading-content",
                div {
                    class: "loading-icon {css_class}",
                    span { class: "material-symbols-outlined", "{icon}" }
                }
                h2 { "{props.title}" }
                p { "{props.message}" }
                if let Some(error_details) = &props.error_details {
                    details {
                        class: "error-details",
                        summary { "Error Detailes" }
                        p { "{error_details}" }
                    }
                }
                div {
                    class: "error-actions",
                    if let Some(on_retry) = props.on_retry {
                        button {
                            class: "retry-button",
                            onclick: move |_| on_retry.call(()),
                            span { class: "material-symbols-outlined", "refresh" }
                            "Retry"
                        }
                    }
                    if props.show_navigation {
                        Link {
                            to: Route::HomePage {},
                            class: "home-button",
                            span { class: "material-symbols-outlined", "home" }
                            "Home"
                        }
                        if props.error_type == "404" {
                            Link {
                                to: Route::ArticlesListPage {},
                                class: "articles-button",
                                span { class: "material-symbols-outlined", "article" }
                                "Articles"
                            }
                        }
                    }
                }
            }
        }
    }
}
