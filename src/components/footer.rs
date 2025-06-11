use leptos::component;
use leptos::prelude::*;

use crate::app::SITE_CONFIGURATION;
use crate::components::layout::ProgressContext;

#[component]
pub fn Footer() -> impl IntoView {
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");
    
    let progress_context = use_context::<ProgressContext>()
        .expect("ProgressContext must be provided");
    let page_loaded = progress_context.0;

    view! {
        <footer class=move || {
            if page_loaded.get() {
                "footer"
            } else {
                "footer animate-fade-in-up"
            }
        }>
            <div class="footer-content">
                <p class="footer-copyright">{format!("(C) {} {}", site_config.copyright_year, site_config.author.name)}</p>
                <p class="footer-powered-by">
                    "Powered by "
                    <a 
                        href="https://github.com/bigsaltyfishes/bigsaltyfishes.github.io"
                    >
                        "Molyuu Blog"
                    </a>
                    " and "
                    <a 
                        href="https://github.com/leptos-rs/leptos"
                    >
                        "Leptos"
                    </a>
                </p>
            </div>
        </footer>
    }
}