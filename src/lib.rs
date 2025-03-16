use cfg_if::cfg_if;
#[cfg(feature = "ssr")]
pub mod api;
pub mod api_types;
#[cfg(feature = "ssr")]
pub mod state;
pub mod ui;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::ui::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount::hydrate_body(Ui);
    }
}}