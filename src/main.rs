#![allow(non_snake_case)]

mod app;
mod bindgen;
mod components;
mod models;
mod pages;
mod router;
mod types;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::launch(app::App);
}
