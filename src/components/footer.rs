use leptos::component;
use leptos::prelude::*;

use crate::app::SITE_CONFIGURATION;

#[component]
pub fn Footer() -> impl IntoView {
    let site_config = SITE_CONFIGURATION
        .get()
        .expect("Site configuration should be loaded by AppLayout");

    view! {
        <footer class="footer">
            <div class="footer-content">
                <p class="footer-copyright">
                    {format!("(C) {} {}", site_config.copyright_year, site_config.author.name)}
                </p>
                <p class="footer-powered-by">
                    "Powered by "
                    <a href="https://github.com/bigsaltyfishes/bigsaltyfishes.github.io">
                        "Molyuu Blog"
                    </a>
                </p>
            </div>
        </footer>
    }
}
