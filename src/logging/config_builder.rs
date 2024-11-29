use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

/// Get Config for Logging
pub fn get_config() -> Config {
    // Create a console appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build();

    // Create a file appender
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("logs/emula80r.log")
        .unwrap();

    // Build the configuration
    Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .logger(Logger::builder().build("emula80r", log::LevelFilter::Info))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(log::LevelFilter::Debug),
        )
        .unwrap()
}
