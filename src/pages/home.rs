use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::SITE_CONFIGURATION;
use crate::components::progress_bar::stop_progress_bar;

#[component]
pub fn HomePage() -> impl IntoView {
    // Get site configuration from global state
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");

    stop_progress_bar();
    let animation_class = RwSignal::new("page-content".to_string());

    Effect::new(move |_| {
        animation_class.set("page-content animate-fade-in-up".to_string());
    });
    view! {
        <Title text=format!("Home - {}", site_config.long()) />
        <div class=move || format!("home-page-container {}", animation_class.get())>
            <div>
                <h1 class="home-page-title">"Welcome"</h1>
                <p class="home-page-text">"Happy Molyuu Everyday🐟."</p>
                <p class="home-page-text">"Navigate using the links in the bar."</p>
            </div>
        </div>
    }
}
