use crate::{router::Route, types::site::Site};
use dioxus::prelude::*;
use once_cell::sync::OnceCell;

pub static SITE_CONFIGURATION: OnceCell<Site> = OnceCell::new();

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
