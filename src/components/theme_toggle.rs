use dioxus::prelude::*;

use crate::components::layout::ThemeContext;

#[component]
pub fn ThemeToggle() -> Element {
    let theme_ctx = use_context::<ThemeContext>();
    let mut is_dark = theme_ctx.0; // is_dark_mode signal
    let current_is_dark = is_dark.read().clone();

    let toggle_theme = move |_| {
        let current_is_dark = is_dark.read().clone();
        is_dark.set(!current_is_dark);
    };

    // Use Material 3 colors with automatic dark mode
    let track_bg = if current_is_dark {
        "bg-primary-container"
    } else {
        "bg-surface-variant"
    };
    
    let (thumb_bg, thumb_icon_color, thumb_position) = if current_is_dark {
        (
            "bg-primary",
            "text-on-primary",
            "theme-toggle-thumb-dark",
        )
    } else {
        (
            "bg-secondary",
            "text-on-secondary",
            "theme-toggle-thumb-light",
        )
    };
    
    rsx! {
        button {
            "aria-label": "Toggle theme",
            class: "theme-toggle-button {track_bg}",
            onclick: toggle_theme,
            
            // Thumb
            div {
                class: "theme-toggle-thumb {thumb_bg} {thumb_position}",
                
                // Light Mode Icon
                span {
                    class: format!(
                        "theme-toggle-icon-container {}",
                        if current_is_dark { "theme-toggle-icon-hidden" } else { "theme-toggle-icon-visible" }
                    ),
                    span {
                        class: "material-symbols-outlined theme-toggle-icon {thumb_icon_color}",
                        "light_mode"
                    }
                }
                
                // Dark Mode Icon
                span {
                    class: format!(
                        "theme-toggle-icon-container {}",
                        if current_is_dark { "theme-toggle-icon-visible" } else { "theme-toggle-icon-hidden" }
                    ),
                    span {
                        class: "material-symbols-outlined theme-toggle-icon {thumb_icon_color}",
                        "dark_mode"
                    }
                }
            }
        }
    }
}
