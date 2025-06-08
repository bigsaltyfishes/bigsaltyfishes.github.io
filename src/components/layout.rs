use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::app::SITE_CONFIGURATION;
use crate::components::error_page::ErrorPage;
use crate::components::navbar::Navbar;
use crate::components::progress_bar::{ProgressBar, ProgressContext};
use crate::router::Route;
use crate::types::site::Site;

#[derive(Clone, Copy, PartialEq)]
pub struct ThemeContext(pub Signal<bool>); // true for dark mode

// This component is used in the router to wrap all pages and provide navbar
#[component]
pub fn AppLayout() -> Element {
    let mut site_resource = use_resource(move || async move { Site::fetch().await });
    let nav_progress_active = use_signal(|| false);

    // Provide ProgressContext to all child components
    use_context_provider(|| ProgressContext(nav_progress_active));

    // Read saved theme state from localStorage, use system preference as fallback
    let is_dark_mode = use_signal(|| {
        let window = web_sys::window().expect("no global `window` exists");
        let storage = window.local_storage().unwrap().unwrap();

        // First try to read from localStorage
        if let Ok(Some(saved_theme)) = storage.get_item("theme") {
            return saved_theme == "dark";
        }

        // If no saved theme exists, use system preference
        window
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .map_or(false, |mql| mql.matches())
    });
    use_context_provider(|| ThemeContext(is_dark_mode));

    // Disable transition animation on init
    use_hook(|| {
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        // Use `no-transition` class defined in CSS to disable transitions initially
        body.class_list().add_1("no-transition").unwrap();

        spawn(async move {
            TimeoutFuture::new(50).await;
            let body = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap();
            body.class_list().remove_1("no-transition").unwrap();
        });
    });

    // Set body class for global styling
    use_effect(move || {
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        let current_is_dark = *is_dark_mode.read();

        // Save theme state to localStorage
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        let theme_value = if current_is_dark { "dark" } else { "light" };
        let _ = storage.set_item("theme", theme_value);

        if current_is_dark {
            body.class_list().add_1("dark").unwrap();
            body.class_list().remove_1("light").unwrap();
        } else {
            body.class_list().add_1("light").unwrap();
            body.class_list().remove_1("dark").unwrap();
        }
    });

    let site_result = site_resource.read();
    match site_result.as_ref() {
        Some(Ok(site)) => {
            // Site config loaded successfully, provide SiteContext and render app
            if SITE_CONFIGURATION.get().is_none() {
                SITE_CONFIGURATION
                    .set(site.clone())
                    .expect("Failed to set site configuration");
            }

            rsx! {
                div {
                    // Overall app container: with background colors, flex column, and full viewport height
                    // Changed from min-h-screen to h-screen to prevent unnecessary scrolling
                    class: "app-layout",
                    ProgressBar { nav_progress_active }
                    Navbar {}
                    // Main content area: now a flex container to center its children, with padding and animation.
                    // The `key` attribute re-triggers the animation on route changes.
                    main {
                        class: "main-content",
                        Outlet::<Route> {}
                    }
                }
            }
        }
        Some(Err(e)) => {
            // Loading error, display error message using ErrorPage component
            rsx! {
                div {
                    class: "app-layout",
                    ProgressBar { nav_progress_active }
                    ErrorPage {
                        title: "Failed to Load Configuration".to_string(),
                        message: "Unable to load site configuration. Please check your network connection and try again.".to_string(),
                        error_details: Some(e.to_string()),
                        on_retry: move |_| site_resource.restart(),
                        error_type: "network".to_string(),
                        show_navigation: false,
                    }
                }
            }
        }
        None => {
            // Loading, only show progress bar
            rsx! {
                div {
                    class: "app-layout",
                    ProgressBar { nav_progress_active }
                }
            }
        }
    }
}
