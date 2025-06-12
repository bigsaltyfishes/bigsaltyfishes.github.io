use gloo_timers::future::TimeoutFuture;
use leptos::attr::global::ClassAttribute;
use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};
use wasm_bindgen_futures::spawn_local;

use crate::{app::SITE_CONFIGURATION, components::theme_toggle::ThemeToggle};

#[derive(Clone, Copy, PartialEq)]
enum CurrentActiveLink {
    Home,
    Articles,
    About,
    None,
}

impl From<String> for CurrentActiveLink {
    fn from(path: String) -> Self {
        match path.as_str() {
            "/" => CurrentActiveLink::Home,
            "/articles" => CurrentActiveLink::Articles,
            "/articles/about" => CurrentActiveLink::About,
            _ => CurrentActiveLink::None, // Default case
        }
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_mobile_menu_open, set_is_mobile_menu_open) = signal(false);
    let (is_closing, set_is_closing) = signal(false);
    let current_active_link = RwSignal::new(CurrentActiveLink::None);

    // Get site configuration
    let site = SITE_CONFIGURATION
        .get()
        .expect("SITE_CONFIGURATION must be initialized before Navbar is rendered"); // Auto-close mobile menu when route changes

    // Helper function to handle the closing logic
    let close_mobile_menu = move || {
        if is_mobile_menu_open.get_untracked() {
            set_is_closing.set(true);
            spawn_local(async move {
                TimeoutFuture::new(100).await;
                set_is_mobile_menu_open.set(false);
                set_is_closing.set(false);
            });
        }
    };

    let location = use_location();
    Effect::new(move |_| {
        let path = location.pathname.get();
        current_active_link.set(CurrentActiveLink::from(path));
        close_mobile_menu();
    });

    let toggle_mobile_menu = move |_| {
        // Open menu
        set_is_mobile_menu_open.set(true);
        set_is_closing.set(false);
    };

    view! {
        <nav class="navbar">
            <div class="navbar-inner">
                <div>
                    <A href="/" attr:class="navbar-brand">
                        <span class="navbar-brand-long">{site.long()}</span>
                        <span class="navbar-brand-short">{site.short()}</span>
                    </A>
                </div>

                <div class="navbar-desktop">
                    <div class="navbar-desktop-links">
                        <A
                            href="/"
                            attr:class=move || {
                                if current_active_link.get() == CurrentActiveLink::Home {
                                    "navbar-desktop-link navbar-desktop-link-active"
                                } else {
                                    "navbar-desktop-link"
                                }
                            }
                        >
                            <span class="material-symbols-outlined navbar-desktop-link-icon">
                                "home"
                            </span>
                            <span>"Home"</span>
                        </A>
                        <A
                            href="/articles"
                            attr:class=move || {
                                if current_active_link.get() == CurrentActiveLink::Articles {
                                    "navbar-desktop-link navbar-desktop-link-active"
                                } else {
                                    "navbar-desktop-link"
                                }
                            }
                        >
                            <span class="material-symbols-outlined navbar-desktop-link-icon">
                                "description"
                            </span>
                            <span>"Articles"</span>
                        </A>
                        <A
                            href="/articles/about"
                            attr:class=move || {
                                if current_active_link.get() == CurrentActiveLink::About {
                                    "navbar-desktop-link navbar-desktop-link-active"
                                } else {
                                    "navbar-desktop-link"
                                }
                            }
                        >
                            <span class="material-symbols-outlined navbar-desktop-link-icon">
                                "info"
                            </span>
                            <span>"About"</span>
                        </A>
                    </div>
                    <div class="navbar-actions">
                        <ThemeToggle />
                    </div>
                </div>

                <button
                    class="navbar-mobile-button"
                    on:click=toggle_mobile_menu
                    aria-label="Toggle mobile menu"
                >
                    <span class="material-symbols-outlined navbar-mobile-button-icon">"menu"</span>
                </button>
            </div>

            <Show when=move || is_mobile_menu_open.get()>
                <div
                    class=move || {
                        format!(
                            "mobile-menu-overlay {}",
                            if is_closing.get() {
                                "mobile-menu-overlay-fade-out"
                            } else {
                                "mobile-menu-overlay-fade-in"
                            },
                        )
                    }
                    on:click=move |_| close_mobile_menu()
                />
                <div class=move || {
                    format!(
                        "mobile-menu-panel {}",
                        if is_closing.get() {
                            "mobile-menu-panel-slide-out"
                        } else {
                            "mobile-menu-panel-slide-in"
                        },
                    )
                }>
                    <div class="mobile-menu-header">
                        <h3 class="mobile-menu-title">"Navigation"</h3>
                        <button
                            class="mobile-menu-close-button"
                            on:click=move |_| close_mobile_menu()
                            aria-label="Close menu"
                        >
                            <span class="material-symbols-outlined mobile-menu-close-icon">
                                "close"
                            </span>
                        </button>
                    </div>
                    <div class="mobile-menu-links">
                        <A
                            href="/"
                            attr:class=move || {
                                if current_active_link.get() == CurrentActiveLink::Home {
                                    "mobile-menu-link mobile-menu-link-active"
                                } else {
                                    "mobile-menu-link"
                                }
                            }
                        >
                            <span class="material-symbols-outlined mobile-menu-link-icon">
                                "home"
                            </span>
                            <span>"Home"</span>
                        </A>
                        <A
                            href="/articles"
                            attr:class=move || {
                                if current_active_link.get() == CurrentActiveLink::Articles {
                                    "mobile-menu-link mobile-menu-link-active"
                                } else {
                                    "mobile-menu-link"
                                }
                            }
                        >
                            <span class="material-symbols-outlined mobile-menu-link-icon">
                                "description"
                            </span>
                            <span>"Articles"</span>
                        </A>
                        <A
                            href="/articles/about"
                            attr:class=move || {
                                if current_active_link.get() == CurrentActiveLink::About {
                                    "mobile-menu-link mobile-menu-link-active"
                                } else {
                                    "mobile-menu-link"
                                }
                            }
                        >
                            <span class="material-symbols-outlined mobile-menu-link-icon">
                                "info"
                            </span>
                            <span>"About"</span>
                        </A>
                    </div>

                    <div class="mobile-menu-footer">
                        <ThemeToggle />
                    </div>
                </div>
            </Show>
        </nav>
    }
}
