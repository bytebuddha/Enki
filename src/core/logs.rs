use log::{error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

pub fn configure_logs(lvl: log::LevelFilter, logfile: &str) {
    let tui = FileAppender::builder().build(logfile).unwrap();
    let rpc = FileAppender::builder()
        .build(&format!("{}.rpc", logfile))
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("tui", Box::new(tui)))
        .appender(Appender::builder().build("rpc", Box::new(rpc)))
        .logger(
            Logger::builder()
                .appender("tui")
                .additive(false)
                .build("enki", lvl),
        )
        .logger(
            Logger::builder()
                .appender("rpc")
                .additive(false)
                .build("crate::xi::client", LevelFilter::Trace),
        )
        .build(Root::builder().appender("tui").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
    std::panic::set_hook(Box::new(|err| {
        error!("{:?}", err);
    }));
}
