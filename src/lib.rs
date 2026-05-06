#![allow(clippy::unused_unit)]

pub mod api;
pub mod app;
mod models;
pub mod utils;
pub mod ws;

mod components;
pub mod routes;

#[cfg(feature = "ssr")]
pub mod db;
#[cfg(feature = "ssr")]
pub mod schema;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
