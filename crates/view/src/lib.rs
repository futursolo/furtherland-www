#![deny(clippy::all)]

mod app;
mod atoms;
mod common;
mod components;
mod contexts;
mod i18n;
mod misc;
mod pages;
mod prelude;

use app::App;
use contexts::Providers;
pub use fl_www_api as api;
use prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <Providers>
            <App />
        </Providers>
    }
}
