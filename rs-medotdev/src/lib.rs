use std::collections::VecDeque;
use cfg_if::cfg_if;
use crate::commands::warehouse::Command;

pub mod app;
pub mod error_template;
pub mod fileserv;

pub mod commands {
    pub mod warehouse;
}

type Item = (usize, String);
pub type Log = VecDeque<Item>;

mod components {
    pub mod sidebar;
    pub mod terminal;
}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}