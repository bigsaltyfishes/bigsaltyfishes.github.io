use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::Outlet;

use crate::app::SITE_CONFIGURATION;
use crate::components::error_page::ErrorPage;
use crate::components::navbar::Navbar;
use crate::components::progress_bar::ProgressBar;
use crate::types::site::Site;

#[derive(Clone, Copy, PartialEq)]
pub struct ProgressContext(pub RwSignal<bool>);

// This component is used in the router to wrap all pages and provide navbar
#[component]
pub fn AppLayout() -> impl IntoView {
    let nav_progress_active = RwSignal::new(false);

    // Provide ProgressContext to all child components
    provide_context(ProgressContext(nav_progress_active));

    // Load site configuration
    let (site_signal, set_site_signal) = signal(None::<Result<Site, String>>);

    // Use spawn_local for client-side only operation
    spawn_local(async move {
        let result = Site::fetch().await;
        set_site_signal.set(Some(result));
    });

    // Disable transition animation on init
    Effect::new(move |_| {
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();

        // Use `no-transition` class defined in CSS to disable transitions initially
        body.class_list().add_1("no-transition").unwrap();

        spawn_local(async move {
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

    view! {
        <div class="app-layout">
            <ProgressBar />
            <Suspense fallback=move || {
                view! { <div class="app-layout"></div> }
            }>
                {move || {
                    site_signal
                        .get()
                        .map(|site_result| {
                            match site_result {
                                Ok(site) => {
                                    if SITE_CONFIGURATION.get().is_none() {
                                        let _ = SITE_CONFIGURATION.set(site);
                                    }
                                    // Site config loaded successfully, set global config and render app
                                    view! {
                                        <Navbar />
                                        <main class="main-content">
                                            <Outlet />
                                        </main>
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    // Loading error, display error message using ErrorPage component
                                    view! {
                                        <ErrorPage
                                            title="Failed to Load Configuration".to_string()
                                            message="Unable to load site configuration. Please check your network connection and try again."
                                                .to_string()
                                            error_details=e.to_string()
                                            error_type="network".to_string()
                                            show_navigation=false
                                        />
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}
