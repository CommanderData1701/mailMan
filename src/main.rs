use log::info;

mod logging;
use logging::initialize_logging;


fn main() {
    initialize_logging();

    info!("Hello, world!");
}
