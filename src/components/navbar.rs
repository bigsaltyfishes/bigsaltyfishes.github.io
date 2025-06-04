use dioxus::prelude::*;
use dioxus_router::prelude::{router, Link};
use gloo_timers::future::TimeoutFuture;

use crate::{components::theme_toggle::ThemeToggle, router::Route, types::site::SiteContext};

#[component]
fn NavbarSeparator() -> Element {
    rsx! { div { class: "navbar-separator" } }
}

#[component]
pub fn Navbar() -> Element {
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut is_closing = use_signal(|| false);
    let nav = router();

    // Get site configuration
    let site_context = use_context::<SiteContext>();
    let site = &site_context.0;

    // Auto-close mobile menu when route changes
    use_effect(move || {
        let _current_route = nav.full_route_string();
        is_mobile_menu_open.set(false);
        is_closing.set(false);
    });

    let toggle_mobile_menu = move |_| {
        let current_state = *is_mobile_menu_open.read();
        if current_state {
            // Start closing animation
            is_closing.set(true);
            // Use spawn to create an async task for the delay
            spawn(async move {
                TimeoutFuture::new(100).await;
                is_mobile_menu_open.set(false);
                is_closing.set(false);
            });
        } else {
            // Open menu
            is_mobile_menu_open.set(true);
            is_closing.set(false);
        }
    };

    let close_mobile_menu = move |_| {
        if *is_mobile_menu_open.read() {
            is_closing.set(true);
            spawn(async move {
                TimeoutFuture::new(100).await;
                is_mobile_menu_open.set(false);
                is_closing.set(false);
            });
        }
    };

    rsx! {
        nav {
            class: "navbar top-navbar",
            div {
                class: "navbar-inner-container",                div {
                    class: "navbar-brand",
                    Link {
                        to: Route::HomePage {},
                        class: "site-title-link",
                        span { class: "site-title-full", "{site.long()}" }
                        span { class: "site-title-short", "{site.short()}" }
                    }
                }

                // Desktop navigation (hidden on mobile)
                div {
                    class: "navbar-right-group desktop-only",
                    div {
                        class: "navbar-links",
                        Link {
                            to: Route::HomePage {},
                            class: "nav-button",
                            active_class: "active",
                            span { class: "material-symbols-outlined", "home" }
                            span { class: "nav-button-text", "Home" }
                        }
                        NavbarSeparator {}
                        Link {
                            to: Route::ArticlesListPage {},
                            class: "nav-button",
                            active_class: "active",
                            span { class: "material-symbols-outlined", "description" }
                            span { class: "nav-button-text", "Articles" }
                        }
                        NavbarSeparator {}
                        Link {
                            to: Route::ArticlePage {
                                id: "about".to_string(),
                            },
                            class: "nav-button",
                            active_class: "active",
                            span { class: "material-symbols-outlined", "info" }
                            span { class: "nav-button-text", "About" }
                        }
                    }
                    div { class: "navbar-actions",
                        ThemeToggle {}
                    }
                }

                // Mobile menu button (hidden on desktop)
                button {
                    class: "mobile-menu-button mobile-only",
                    onclick: toggle_mobile_menu,
                    aria_label: "Toggle mobile menu",
                    span { class: "material-symbols-outlined", "menu" }
                }                // Mobile menu overlay
                if *is_mobile_menu_open.read() {
                    {
                        let overlay_class = if *is_closing.read() {
                            "mobile-menu-overlay closing"
                        } else {
                            "mobile-menu-overlay"
                        };
                        let menu_class = if *is_closing.read() {
                            "mobile-menu closing"
                        } else {
                            "mobile-menu"
                        };

                        rsx! {
                            div {
                                class: "{overlay_class}",
                                onclick: close_mobile_menu,
                                div {
                                    class: "{menu_class}",
                                    onclick: |e| e.stop_propagation(), // Prevent closing when clicking inside menu

                                    // Mobile menu header with close button
                                    div {
                                        class: "mobile-menu-header",
                                        h3 { class: "mobile-menu-title", "Navigation" }
                                        button {
                                            class: "mobile-menu-close",
                                            onclick: close_mobile_menu,
                                            "aria-label": "Close menu",
                                            span { class: "material-symbols-outlined", "close" }
                                        }
                                    }

                                    div {
                                        class: "mobile-menu-links",
                                        Link {
                                            to: Route::HomePage {},
                                            class: "mobile-nav-button",
                                            active_class: "active",
                                            onclick: close_mobile_menu,
                                            span { class: "material-symbols-outlined", "home" }
                                            span { "Home" }
                                        }
                                        Link {
                                            to: Route::ArticlesListPage {},
                                            class: "mobile-nav-button",
                                            active_class: "active",
                                            onclick: close_mobile_menu,
                                            span { class: "material-symbols-outlined", "description" }
                                            span { "Articles" }
                                        }
                                        Link {
                                            to: Route::ArticlePage {
                                                id: "about".to_string(),
                                            },
                                            class: "mobile-nav-button",
                                            active_class: "active",
                                            onclick: close_mobile_menu,
                                            span { class: "material-symbols-outlined", "info" }
                                            span { "About" }
                                        }
                                    }
                                    div {
                                        class: "mobile-menu-theme-toggle",
                                        ThemeToggle {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
