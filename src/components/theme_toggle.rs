use dioxus::prelude::*;

use crate::components::layout::ThemeContext;

#[component]
pub fn ThemeToggle() -> Element {
    let theme_ctx = use_context::<ThemeContext>();
    let mut is_dark = theme_ctx.0; // is_dark_mode signal
    let current_is_dark = is_dark.read().clone();
    let toggle_theme = move |_| {
        let current_is_dark = is_dark.read().clone();
        let new_is_dark = !current_is_dark;
        is_dark.set(new_is_dark);
    };
    let switch_class = if current_is_dark {
        "theme-switch active"
    } else {
        "theme-switch"
    };
    rsx! {
        button {
            "aria-label": "Toggle theme",
            class: "{switch_class}",
            onclick: toggle_theme,
            div { class: "switch-track",                div {
                    class: "switch-thumb",
                    // Use two icons, control their visibility and transitions via CSS
                    span {
                        class: "icon-container light-icon",
                        span {
                            class: "material-symbols-outlined",
                            "light_mode"
                        }
                    }
                    span {
                        class: "icon-container dark-icon",
                        span {
                            class: "material-symbols-outlined",
                            "dark_mode"
                        }
                    }
                }
            }
        }
    }
}
