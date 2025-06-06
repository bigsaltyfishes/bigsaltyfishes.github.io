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
    let router = router();

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

        body.class_list().add_1("no-theme-transition").unwrap();

        spawn(async move {
            TimeoutFuture::new(50).await;
            let body = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap();
            body.class_list().remove_1("no-theme-transition").unwrap();
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
            body.class_list().add_1("dark-mode").unwrap();
            body.class_list().remove_1("light-mode").unwrap();
        } else {
            body.class_list().add_1("light-mode").unwrap();
            body.class_list().remove_1("dark-mode").unwrap();
        }
    });
    
    use_effect(move || {
        let site_result = site_resource.read();
        if let Some(Ok(site)) = site_result.as_ref() {
            let site_name = site.long();
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                document.set_title(&site_name);
            }
        }
    });

    let current_route = router.full_route_string();
    let mut animation_class = use_signal(|| "");
    use_effect(use_reactive((&current_route,), move |(_,)| {
        spawn(async move {
            // Reset progress bar animation state
            animation_class.set("");
            TimeoutFuture::new(10).await;
            animation_class.set("active");
        });
    }));

    let site_result = site_resource.read();
    match site_result.as_ref() {
        Some(Ok(site)) => {
            // Site config loaded successfully, provide SiteContext and render app
            if SITE_CONFIGURATION.get().is_none() {
                // Set the site configuration globally
                SITE_CONFIGURATION
                    .set(site.clone())
                    .expect("Failed to set site configuration");
            }
            
            rsx! {
                // The progress bar is now at the very top of the app container
                div {
                    class: "app-container",
                    ProgressBar { nav_progress_active } // Use componentized progress bar
                    Navbar { } // Pass an hide signal to progress bar
                    div { class: "content-area {animation_class.read()}", Outlet::<Route> {} }
                }
            }
        }
        Some(Err(e)) => {
            // Loading error, display error message using ErrorPage component
            rsx! {
                div {
                    class: "app-container",
                    ProgressBar { nav_progress_active } // Use componentized progress bar
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
                    class: "app-container",
                    ProgressBar { nav_progress_active } // Use componentized progress bar
                }
            }
        }
    }
}
