pub enum Status {
    OK,
    ERROR,
}

impl Status {
    pub fn get_status_header(&self) -> String {
        let http_version = "HTTP/1.1";
        let status = match self {
            Status::OK => "200 OK",
            Status::ERROR => "404 ERROR",
        };

        format!("{http_version} {status}")
    }
}
