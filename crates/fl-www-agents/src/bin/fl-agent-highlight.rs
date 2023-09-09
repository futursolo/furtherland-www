use yew_agent::PublicWorker;

fn main() {
    use log::Level;
    console_error_panic_hook::set_once();

    #[cfg(debug_assertions)]
    console_log::init_with_level(Level::Debug).expect("Failed to initialise Log!");
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(Level::Error).expect("Failed to initialise Log!");

    fl_www_agents::highlight::Worker::register();
}
