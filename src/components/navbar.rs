use dioxus::prelude::*;
use dioxus_router::prelude::{router, Link};
use gloo_timers::future::TimeoutFuture;

use crate::{app::SITE_CONFIGURATION, components::theme_toggle::ThemeToggle, router::Route};

#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut is_closing = use_signal(|| false);
    let nav = router();

    // Get site configuration
    let site = SITE_CONFIGURATION
        .get()
        .expect("SITE_CONFIGURATION must be initialized before Navbar is rendered");

    // Auto-close mobile menu when route changes
    use_effect(move || {
        let _current_route = nav.full_route_string();
        is_mobile_menu_open.set(false);
        is_closing.set(false);
    });

    // A helper function to handle the closing logic
    let mut close_mobile_menu = move || {
        if *is_mobile_menu_open.read() {
            is_closing.set(true);
            spawn(async move {
                TimeoutFuture::new(100).await;
                is_mobile_menu_open.set(false);
                is_closing.set(false);
            });
        }
    };

    let toggle_mobile_menu = move |_| {
        let current_state = *is_mobile_menu_open.read();
        if current_state {
            close_mobile_menu();
        } else {
            // Open menu
            is_mobile_menu_open.set(true);
            is_closing.set(false);
        }
    };    rsx! {
        nav {
            class: "navbar",
            
            // Inner container with max-width, padding, and flexbox layout
            div {
                class: "navbar-inner",
                  // --- Site Brand / Title (Always visible, on the left) ---
                div {
                    Link {
                        to: Route::HomePage {},
                        class: "navbar-brand",
                        // Show full title on desktop, short on mobile
                        span { class: "navbar-brand-long", "{site.long()}" }
                        span { class: "navbar-brand-short", "{site.short()}" }
                    }
                }                // --- Desktop Navigation (Hidden on mobile) ---
                div {
                    class: "navbar-desktop",
                    div {
                        class: "navbar-desktop-links",
                        Link {
                            to: Route::HomePage {},
                            class: "navbar-desktop-link",
                            active_class: "navbar-desktop-link-active",
                            span { class: "material-symbols-outlined navbar-desktop-link-icon", "home" }
                            span { "Home" }
                        }
                        Link {
                            to: Route::ArticlesListPage {},
                            class: "navbar-desktop-link",
                            active_class: "navbar-desktop-link-active",
                            span { class: "material-symbols-outlined navbar-desktop-link-icon", "description" }
                            span { "Articles" }
                        }
                        Link {
                            to: Route::ArticlePage { id: "about".to_string() },
                            class: "navbar-desktop-link",
                            active_class: "navbar-desktop-link-active",
                            span { class: "material-symbols-outlined navbar-desktop-link-icon", "info" }
                            span { "About" }
                        }
                    }
                    div { class: "navbar-actions", // Navbar actions
                        ThemeToggle {}
                    }
                }                // --- Mobile Menu Button (Hidden on desktop) ---
                button {
                    class: "navbar-mobile-button", // Negative margin to align to the very edge
                    onclick: toggle_mobile_menu,
                    aria_label: "Toggle mobile menu",
                    span { class: "material-symbols-outlined navbar-mobile-button-icon", "menu" }
                }
            }
              // --- Mobile Menu Overlay & Panel ---
            if *is_mobile_menu_open.read() {
                // Overlay
                div {
                    class: format!(
                        "mobile-menu-overlay {}",
                        if *is_closing.read() { "mobile-menu-overlay-fade-out" } else { "mobile-menu-overlay-fade-in" }
                    ),
                    onclick: move |_| close_mobile_menu(),
                }
                // Menu Panel (Now positioned on the right)
                div {
                    class: format!(
                        "mobile-menu-panel {}",
                        if *is_closing.read() { "mobile-menu-panel-slide-out" } else { "mobile-menu-panel-slide-in" }
                    ),
                    onclick: |e| e.stop_propagation(),

                    // Mobile Menu Header
                    div {
                        class: "mobile-menu-header",
                        h3 { class: "mobile-menu-title", "Navigation" }
                        button {
                            class: "mobile-menu-close-button",
                            onclick: move |_| close_mobile_menu(),
                            aria_label: "Close menu",
                            span { class: "material-symbols-outlined mobile-menu-close-icon", "close" }
                        }
                    }                    
                    // Mobile Menu Links
                    div {
                        class: "mobile-menu-links",
                        Link {
                            to: Route::HomePage {},
                            class: "mobile-menu-link",
                            active_class: "mobile-menu-link-active",
                            span { class: "material-symbols-outlined mobile-menu-link-icon", "home" }
                            span { "Home" }
                        }
                        Link {
                            to: Route::ArticlesListPage {},
                            class: "mobile-menu-link",
                            active_class: "mobile-menu-link-active",
                            span { class: "material-symbols-outlined mobile-menu-link-icon", "description" }
                            span { "Articles" }
                        }
                        Link {
                            to: Route::ArticlePage { id: "about".to_string() },
                            class: "mobile-menu-link",
                            active_class: "mobile-menu-link-active",
                            span { class: "material-symbols-outlined mobile-menu-link-icon", "info" }
                            span { "About" }
                        }
                    }

                    // Mobile Menu Footer (Theme Toggle)
                    div {
                        class: "mobile-menu-footer",
                        ThemeToggle {}
                    }
                }
            }
        }
    }
}
