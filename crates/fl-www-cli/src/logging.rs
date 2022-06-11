use std::sync::Once;

use env_logger::Builder;
use log::LevelFilter;

static INIT: Once = Once::new();

pub(crate) fn init() {
    INIT.call_once(|| {
        #[cfg(debug_assertions)]
        let default_level = LevelFilter::Debug;
        #[cfg(not(debug_assertions))]
        let default_level = LevelFilter::Info;

        Builder::new().filter_level(default_level).init();
    });
}
