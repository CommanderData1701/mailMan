use log::info;

mod logging;
use logging::initialize_logging;


fn main() {
    initialize_logging().unwrap();


    log::info!("Hello, world!");
}
