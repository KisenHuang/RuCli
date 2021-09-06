pub extern crate log;

use chrono::Local;

use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

///初始化方法
pub fn init<S: Into<String>>(tag: S, call: fn(String, String, String, String)) -> Result<(), SetLoggerError> {
    SimpleLogger::new(tag.into(), call).init()
}

struct SimpleLogger {
    time_format: &'static str,
    tag: String,
    log_call: fn(String, String, String, String),
}

impl SimpleLogger {
    pub fn new(tag: String, call: fn(String, String, String, String)) -> SimpleLogger {
        SimpleLogger {
            time_format: "%Y-%m-%d %H:%M:%S.%3f",
            tag,
            log_call: call,
        }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_logger(Box::leak(Box::new(self)))
            .map(|()| log::set_max_level(LevelFilter::Info))
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            (self.log_call)(
                Local::now().format(self.time_format).to_string(),
                record.level().to_string(),
                self.tag.clone(),
                format!("|{}| {}", record.target(), record.args()).to_string(),
            );
        }
    }
    fn flush(&self) {}
}