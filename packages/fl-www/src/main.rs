#![deny(clippy::all)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod contexts;
mod i18n;
mod metadata;
mod misc;
mod pages;
mod prelude;
mod store;
mod tmpfs;

use prelude::*;

use app::App;
use contexts::Providers;

struct Root;

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Providers>
                <App />
            </Providers>
        }
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
