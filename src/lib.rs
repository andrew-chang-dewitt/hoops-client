pub mod app;
pub mod components;
pub mod error_template;
pub mod fileserv;

use cfg_if::cfg_if;
use leptos::*;

cfg_if! { if #[cfg(feature = "hydrate")] {

    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}
