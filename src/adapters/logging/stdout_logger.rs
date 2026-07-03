use crate::core::ports::LoggerPort;

pub struct StdoutLogger;

impl LoggerPort for StdoutLogger {
    fn info(&self, msg: &str) {
        println!("{msg}");
    }
    fn warn(&self, msg: &str) {
        eprintln!("WARN: {msg}");
    }
    fn error(&self, msg: &str) {
        eprintln!("ERROR: {msg}");
    }
}
