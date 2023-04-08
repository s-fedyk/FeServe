use std::fmt::{Display, Formatter};
use std::net::TcpStream;
use std::str::from_utf8;
use std::io::Write;
use crate::http::status_code;
use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Response<'response_buffer> {
    status_code: StatusCode,
    body: Option<&'response_buffer str> // optional, we might have no body bc fail
}

impl<'response_buffer> Response<'response_buffer> {
    pub fn new(status_code :StatusCode, buffer :Option<&'response_buffer[u8]>) -> Self {
        let mut body = None;

        match buffer {
            Some(response_body) => {
                if let Ok(computed_body) = from_utf8(response_body) {
                    body = Some(computed_body);
                }
            }
            _ => {let status_code = StatusCode::ServerError;}
        }

        Response{
            status_code,
            body
        }
    }

    /**
    * Reply to TCP stream with built socket
    **/
    pub fn reply(&self, stream: TcpStream) -> std::io::Result<()> {
        write!(&stream, "{}", self)
    }
}

impl<'response_buffer> Display for Response<'response_buffer> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {} {} \n\r\n\r{}", self.status_code, self.status_code.get_reason_code(), self.body.unwrap_or(""))
    }
}