use log::info;

mod logging;
mod mrule_parser;

use logging::initialize_logging;


fn main() {
    initialize_logging();

    info!("Hello, world!");
}


// #[cfg(test)]
// mod tests;
