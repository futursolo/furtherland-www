use std::sync::Once;

use log::Level;
use worker::{Date, Request};

static INIT: Once = Once::new();

pub(crate) fn init() {
    INIT.call_once(|| {
        // Optionally, get more helpful error messages written to the console in the case of a panic.
        console_error_panic_hook::set_once();

        #[cfg(debug_assertions)]
        console_log::init_with_level(Level::Debug).expect("Failed to initialise Log!");
        #[cfg(not(debug_assertions))]
        console_log::init_with_level(Level::Info).expect("Failed to initialise Log!");
    });
}

pub(crate) fn log_request(req: &Request) {
    log::debug!(
        "{} - [{}], Region: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}
