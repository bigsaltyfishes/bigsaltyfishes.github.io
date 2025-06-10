use leptos::prelude::*;

use crate::components::layout::ThemeContext;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_ctx = expect_context::<ThemeContext>();
    let is_dark = theme_ctx.0; // is_dark_mode signal

    let toggle_theme = move |_| {
        let current_is_dark = is_dark.get();
        is_dark.set(!current_is_dark);
    };

    // Use Material 3 colors with automatic dark mode
    let track_bg = move || {
        if is_dark.get() {
            "bg-primary-container"
        } else {
            "bg-surface-variant"
        }
    };

    let thumb_classes = move || {
        if is_dark.get() {
            ("bg-primary", "text-on-primary", "theme-toggle-thumb-dark")
        } else {
            (
                "bg-secondary",
                "text-on-secondary",
                "theme-toggle-thumb-light",
            )
        }
    };

    view! {
        <button
            aria-label="Toggle theme"
            class=move || format!("theme-toggle-button {}", track_bg())
            on:click=toggle_theme
        >
            <div class=move || {
                let (thumb_bg, _, thumb_position) = thumb_classes();
                format!("theme-toggle-thumb {} {}", thumb_bg, thumb_position)
            }>
                // Light Mode Icon
                <span class=move || {
                    let current_is_dark = is_dark.get();
                    format!(
                        "theme-toggle-icon-container {}",
                        if current_is_dark {
                            "theme-toggle-icon-hidden"
                        } else {
                            "theme-toggle-icon-visible"
                        },
                    )
                }>
                    <span class=move || {
                        let (_, thumb_icon_color, _) = thumb_classes();
                        format!("material-symbols-outlined theme-toggle-icon {}", thumb_icon_color)
                    }>"light_mode"</span>
                </span>

                // Dark Mode Icon
                <span class=move || {
                    let current_is_dark = is_dark.get();
                    format!(
                        "theme-toggle-icon-container {}",
                        if current_is_dark {
                            "theme-toggle-icon-visible"
                        } else {
                            "theme-toggle-icon-hidden"
                        },
                    )
                }>
                    <span class=move || {
                        let (_, thumb_icon_color, _) = thumb_classes();
                        format!("material-symbols-outlined theme-toggle-icon {}", thumb_icon_color)
                    }>"dark_mode"</span>
                </span>
            </div>
        </button>
    }
}
