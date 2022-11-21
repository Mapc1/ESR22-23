use std::sync::{Arc, LockResult, Mutex};
use super::severity::Severity;

#[derive(Copy, Clone)]
struct Settings {
    info: bool,
    error: bool,
    debug: bool,
}

#[derive(Clone)]
pub struct Logger {
    logger: Arc<Mutex<Settings>>
}

impl Logger {
    pub fn new(info: bool, error: bool, debug: bool) -> Logger {
        Logger {
            logger: Arc::new(Mutex::new(Settings{
                info,
                error,
                debug
            }))
        }
    }

    pub fn log_info(&self, msg: String) -> Result<(), String> {
        let info = match self.logger.lock() {
            Ok(settings) => settings.info,
            Err(err) => return Err(err.to_string())
        };

        if info {
            println!("{}: {}", Severity::INFO.stamp(), msg);
        }
        Ok(())
    }

    pub fn log_error(&self, msg: String) -> Result<(), String> {
        let error = match self.logger.lock() {
            Ok(settings) => settings.error,
            Err(err) => return Err(err.to_string())
        };

        if error {
            eprintln!("{}: {}", Severity::ERROR.stamp(), msg);
        }
        Ok(())
    }
    
    pub fn log_dbg(&self, msg: String) -> Result<(), String> {
        let debug = match self.logger.lock() {
            Ok(settings) => settings.debug,
            Err(err) => return Err(err.to_string())
        };

        if debug {
            println!("{}: {}", Severity::DEBUG.stamp(), msg);
        }
        Ok(())
    }
}