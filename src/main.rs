#![allow(non_snake_case)]

use leptos::prelude::*;

mod app;
mod bindgen;
mod components;
mod models;
mod pages;
mod router;
mod types;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(app::App);
}
