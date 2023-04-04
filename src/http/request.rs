use super::method::Method;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl Request {

}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest, // bad request
    InvalidEncoding, // encoding of incorrect type
    InvalidProtocol, //only support http 1.1
    InvalidMethod // method that we don't implement
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ParseError {

}
