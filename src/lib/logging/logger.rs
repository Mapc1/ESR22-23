use super::severity::Severity;

pub struct Logger {
    info: bool,
    error: bool,
    debug: bool,
}

impl Logger {
    pub fn new(info: bool, error: bool, debug: bool) -> Logger {
        Logger {
            info,
            error,
            debug,
        }
    }

    pub fn log_info(&self, msg: String) {
        if self.info {
            println!("{}: {}", Severity::INFO.stamp(), msg);
        }
    }

    pub fn log_error(&self, msg: String) {
        if self.error {
            eprintln!("{}: {}", Severity::ERROR.stamp(), msg);
        }
    }
    
    pub fn log_dbg(&self, msg: String) {
        if self.debug {
            println!("{}: {}", Severity::DEBUG.stamp(), msg);
        }
    }
}