use leptos::*;

pub mod app;
pub mod pages;
pub mod components;
pub mod api;
pub mod state;
pub mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| {
        view! { <app::App/> }
    });
}


