
//import Method enum from http module
use super::method::Method;
use std::convert::TryFrom;
use::std::error::Error;
use::std::fmt::{Result as FmtResult, Display, Formatter, Debug};


pub struct Request {

    path: String,
    query: String,
    method: Method,
}

impl Request {


}

//trait implementation
impl TryFrom<&[u8]> for Request {

    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {

    Invalidrequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {

    fn message(&self) -> &str {
        match self {
            Self::Invalidrequest=> "Invalid request",
            Self::InvalidEncoding=> "Invalid encoding",
            Self::InvalidProtocol=> "Invalid protocol",
            Self::InvalidMethod=> "Invalid method",
        }
    }
}

impl Display for ParseError {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {


}


/*
trait Encrypt {

    fn encrypt(&self) -> Self {
        unimplemented!()
    }
    
}

impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}
*/