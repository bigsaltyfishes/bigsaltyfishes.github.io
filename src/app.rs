use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use once_cell::sync::OnceCell;

use crate::router::AppRouter;
use crate::types::site::Site;

pub static SITE_CONFIGURATION: OnceCell<Site> = OnceCell::new();

#[derive(Clone, Copy, PartialEq)]
pub struct ThemeContext(pub RwSignal<bool>); // true for dark mode

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Read saved theme state from localStorage, use system preference as fallback
    let is_dark_mode = RwSignal::new({
        let window = web_sys::window().expect("no global `window` exists");
        let storage = window.local_storage().unwrap().unwrap();

        // First try to read from localStorage
        if let Ok(Some(saved_theme)) = storage.get_item("theme") {
            saved_theme == "dark"
        } else {
            // If no saved theme exists, use system preference
            window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
                .map_or(false, |mql| mql.matches())
        }
    });
    provide_context(ThemeContext(is_dark_mode));

    // Set body class for global styling
    Effect::new(move |_| {
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        let current_is_dark = is_dark_mode.get();

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

    view! { <AppRouter /> }
}
