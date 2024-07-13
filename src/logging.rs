use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use std::env;

const LOGFILE: &str = "/var/log/mailMan.log";

pub fn initialize_logging() -> Result<(), E> {
    // Determine log output based on debug mode
    let mut builder = Config::builder();
    
    if cfg!(debug_assertions) {
        // Output to the console in debug mode
        builder = builder.appender(
            Appender::builder()
                .build("stdout", Box::new(log4rs::append::console::ConsoleAppender::builder().build())),
        );
    } else {
        // Output to a file in release mode
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build(LOGFILE)?;

        builder = builder.appender(
            Appender::builder()
                .build("logfile", Box::new(logfile)),
        );
    }

    // Build the root logger configuration
    let config = builder.build(
        Root::builder()
            .appender(if cfg!(debug_assertions) { "stdout" } else { "logfile" })
            .build(LevelFilter::Info)
    )?;

    // Initialize log4rs with the constructed configuration
    log4rs::init_config(config)?;

    Ok(())
}
