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
    stop_progress_bar();    let (icon, icon_color_class) = match props.error_type.as_str() {
        "404" => ("unknown_document", "error-page-icon-404"),
        "500" => ("error", "error-page-icon-500"),
        "network" => ("wifi_off", "error-page-icon-network"),
        _ => ("error", "error-page-icon-default"),
    };rsx! {
        // Main container for the error page
        div {
            class: "error-page-container",
            
            // Content box
            div {
                class: "error-page-content",
                
                // Icon
                div {
                    class: "error-page-icon-container",                    span {
                        class: "material-symbols-outlined error-page-icon {icon_color_class}",
                        "{icon}"
                    }
                }
                
                // Title and Message
                h2 {
                    class: "error-page-title",
                    "{props.title}"
                }
                p {
                    class: "error-page-message",
                    "{props.message}"
                }
                
                // Error Details (collapsible)
                if let Some(error_details) = &props.error_details {
                    details {
                        class: "error-page-details",
                        summary {
                            class: "error-page-details-summary",
                            "Error Details"
                        }
                        p {
                            class: "error-page-details-content",
                            "{error_details}"
                        }
                    }
                }

                // Action Buttons
                div {
                    class: "error-page-actions",
                    
                    if let Some(on_retry) = props.on_retry {
                        button {
                            class: "error-page-button error-page-button-primary",
                            onclick: move |_| on_retry.call(()),
                            span { class: "material-symbols-outlined error-page-button-icon", "refresh" }
                            "Retry"
                        }
                    }
                    
                    if props.show_navigation {
                        Link {
                            to: Route::HomePage {},
                            class: "error-page-button error-page-button-secondary",
                            span { class: "material-symbols-outlined error-page-button-icon", "home" }
                            "Home"
                        }
                        if props.error_type == "404" {
                            Link {
                                to: Route::ArticlesListPage {},
                                class: "error-page-button error-page-button-tertiary",
                                span { class: "material-symbols-outlined error-page-button-icon", "article" }
                                "Articles"
                            }
                        }
                    }
                }
            }
        }
    }
}
