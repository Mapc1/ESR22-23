pub enum Severity {
    INFO,
    DEBUG,
    ERROR
}

impl Severity {
    pub fn stamp(&self) -> &str {
        match self {
            Severity::INFO => "[INFO]",
            Severity::DEBUG => "[DEBUG]",
            Severity::ERROR => "[ERROR]",
        }
    }
}