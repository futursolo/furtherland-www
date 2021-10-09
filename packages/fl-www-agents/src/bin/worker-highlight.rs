use yew_agent::Threaded;

fn main() {
    use log::Level;

    #[cfg(debug_assertions)]
    console_log::init_with_level(Level::Debug).expect("Failed to initialise Log!");
    #[cfg(not(debug_assertions))]
    console_log::init_with_level(Level::Error).expect("Failed to initialise Log!");

    fl_www_agents::highlight::Worker::register();
}
