use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressContext(pub Signal<bool>);

#[derive(Props, Clone, PartialEq)]
pub struct ProgressBarProps {
    pub nav_progress_active: Signal<bool>,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    let nav_progress_active = props.nav_progress_active;
    let router = dioxus_router::hooks::router();

    // Monitor route changes to trigger the progress bar animation
    let current_route = router.full_route_string();
    use_effect(use_reactive((&current_route,), move |(_route_string,)| {
        // This effect triggers whenever the route changes
        let mut nav_progress_active = nav_progress_active.clone();
        spawn(async move {
            if *nav_progress_active.peek() {
                // If animation was somehow stuck true, reset
                nav_progress_active.set(false);
                // Brief pause to ensure CSS can pick up the change if re-triggering fast
                TimeoutFuture::new(10).await;
            }
            nav_progress_active.set(true); // Activate progress bar
        });
    }));    // Dynamically set width and transition classes based on the active state
    let progress_class = if *nav_progress_active.read() {
        // Active state: full width with a longer ease-out transition
        "progress-bar-active"
    } else {
        // Inactive state: zero width with a shorter ease-out transition
        "progress-bar-inactive"
    };

    rsx! {
        // Progress bar container: fixed position, full width, specific height, and z-index
        div {
            class: "progress-bar-container",
            // The actual progress bar element with dynamic width and color
            div {
                class: "{progress_class}"
            }
        }
    }
}


pub fn stop_progress_bar() {
    // Try to get the context, but don't panic if it's not available
    if let Some(progress_context) = try_use_context::<ProgressContext>() {
        let mut nav_progress_active = progress_context.0;
        spawn(async move {
            TimeoutFuture::new(400).await; // Wait for 400ms before hiding
            nav_progress_active.set(false); // Deactivate progress bar
        });
    }
}
