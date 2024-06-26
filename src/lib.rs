#![allow(non_snake_case)]

use cfg_if::cfg_if;
pub mod app;
pub mod components;
pub mod constants;
pub mod content;
pub mod error_template;
pub mod fileserv;
pub mod global_state;
pub mod page;
pub mod svg_defs;
pub mod utils;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App /> }
        });
    }
}}
