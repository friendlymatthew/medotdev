mod app;
pub mod autocomplete;
use leptos_router::IntoParam;
use leptos_router::Params;

mod components {
    pub mod sidebar;
    pub mod terminal;
}

pub mod commands {
    pub mod warehouse;
}

use app::*;
use leptos::*;
use std::collections::VecDeque;

type Item = (usize, String);
pub type Log = VecDeque<Item>;

#[derive(Params, PartialEq)]
pub struct CmdParam {
    cmd: Option<String>,
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App/> }
    });
}
