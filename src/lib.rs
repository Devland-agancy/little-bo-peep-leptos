#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub mod app;
pub mod components;
pub mod constants;
pub mod global_state;
pub mod page;
pub mod svg_defs;
pub mod utils;

#[cfg(feature = "ssr")]
pub mod fallback;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
