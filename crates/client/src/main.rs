#![deny(clippy::all)]
#![deny(missing_debug_implementations)]

use {fl_www_api as api, fl_www_view as view};

mod app;
use api::FrontendBridge;
use app::App;
use tracing_subscriber::filter::LevelFilter;

fn main() {
    // Configures Logging
    stellation_frontend::trace::init_default(LevelFilter::INFO);

    // Starts Application
    stellation_frontend::Renderer::<App>::new()
        .bridge_selector::<FrontendBridge, _>()
        .render();
}
