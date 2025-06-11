use leptos_router::components::A;
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title};

use crate::app::SITE_CONFIGURATION;
use crate::components::progress_bar::stop_progress_bar;

#[component]
pub fn HomePage() -> impl IntoView {
    // Get site configuration from global state
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");

    let welcome_title = site_config.home.welcome_title.clone();
    let welcome_text = site_config.home.welcome_text
        .clone()
        .into_iter()
        .map(|text| {
            view! {
                <p class="home-page-text">{text}</p>
            }
        })
        .collect_view();

    stop_progress_bar();
    let animation_class = RwSignal::new("page-content".to_string());
    Effect::new(move |_| {
        animation_class.set("page-content animate-fade-in-up".to_string());
    });

    view! {
        <Title text=format!("Home - {}", site_config.long()) />
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/brands.min.css" />
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/fontawesome.min.css" />
        <div class=move || format!("home-page-container {}", animation_class.get())>
            <div>
                <h1 class="home-page-title">{welcome_title}</h1>
                { welcome_text }
                <p class="home-page-text mt-4">
                    <div class="items-center flex flex-row justify-center gap-4">
                        <A
                            attr:class="text-[20px]"
                            href=move || format!("https://github.com/{}", site_config.author.github)
                        >
                            <i class="fa-brands fa-github"></i>
                        </A>
                        <A 
                            attr:class="material-symbols-outlined text-2xl"
                            href=move || format!("mailto:{}", site_config.author.email)
                        >
                            "mail"
                        </A>
                    </div>
                </p>
            </div>
        </div>
    }
}
