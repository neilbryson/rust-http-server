use super::status_code::StatusCode;
use super::headers::Headers;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response<'res> {
    body: Option<String>,
    headers: Option<Headers<'res>>,
    status_code: StatusCode,
}

impl<'res> Response<'res> {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        let headers = Headers::from("Server: rust-http-server\r\n");
        Response { body, headers: Some(headers), status_code }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        let headers = match &self.headers {
            Some(h) => h.to_string(),
            None => "".to_string(),
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n{} \r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            headers,
            body,
        )
    }
}
