mod app;
pub mod autocomplete;
use leptos_router::Params;
use leptos_router::IntoParam;

mod components {
    pub mod sidebar;
    pub mod terminal;
}


pub mod commands {
    pub mod warehouse;
}

use std::collections::VecDeque;
use app::*;
use leptos::*;

type Item = (usize, String);
pub type Log = VecDeque<Item>;

#[derive(Params, PartialEq)]
pub struct CmdParam {
    cmd: Option<String>
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App/> }
    });
}