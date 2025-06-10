use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_location;

use crate::components::layout::ProgressContext;

#[component]
pub fn ProgressBar() -> impl IntoView {
    let location = use_location();
    let nav_progress_active = use_context::<ProgressContext>()
        .expect("ProgressContext must be provided")
        .0;

    // Monitor route changes to trigger the progress bar animation
    Effect::new(move |_| {
        let _pathname = location.pathname.get();
        spawn_local(async move {
            if nav_progress_active.get_untracked() {
                // If animation was somehow stuck true, reset
                nav_progress_active.set(false);
                // Brief pause to ensure CSS can pick up the change if re-triggering fast
                TimeoutFuture::new(10).await;
            }
            nav_progress_active.set(true); // Activate progress bar
        });
    });

    // Dynamically set width and transition classes based on the active state
    let progress_class = move || {
        if nav_progress_active.get() {
            // Active state: full width with a longer ease-out transition
            "progress-bar-active"
        } else {
            // Inactive state: zero width with a shorter ease-out transition
            "progress-bar-inactive"
        }
    };

    view! {
        <div class="progress-bar-container">
            <div class=progress_class></div>
        </div>
    }
}

pub fn stop_progress_bar() {
    // Try to get the context, but don't panic if it's not available
    if let Some(progress_context) = use_context::<ProgressContext>() {
        let nav_progress_active = progress_context.0;
        spawn_local(async move {
            TimeoutFuture::new(400).await; // Wait for 400ms before hiding
            nav_progress_active.set(false); // Deactivate progress bar
        });
    }
}
