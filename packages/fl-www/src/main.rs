#![deny(clippy::all)]

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod atoms;
mod common;
mod components;
mod contexts;
mod i18n;
mod misc;
mod pages;
mod prelude;

use prelude::*;

use app::App;
use contexts::Providers;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <Providers>
            <App />
        </Providers>
    }
}

pub fn main() {
    use log::Level;

    #[cfg(debug_assertions)]
    console_log::init_with_level(Level::Debug).expect("Failed to initialise Log!");
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(Level::Error).expect("Failed to initialise Log!");

    yew::start_app::<Root>();
}
