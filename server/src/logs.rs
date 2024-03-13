use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn init_log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}",
        )))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
