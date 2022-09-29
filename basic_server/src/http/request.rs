
//import Method enum from http module
use super::method::Method;
use std::convert::TryFrom;
use::std::error::Error;
use::std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use::std::str;
use::std::str::Utf8Error;


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

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n... HEADERS
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        /*
        match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding),
        }
        

        //same thing as above
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {},
            Err(e) => return Err(e),
        }
        */

        //same thing again
        //let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        let request = str::from_utf8(buf)?;  //works as well beacuse Utf8Error is handled in the From trait

        //variable shadowing (overriding)
        let (method, request) = get_next_word(request).ok_or(ParseError::Invalidrequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::Invalidrequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::Invalidrequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        unimplemented!()

    }
}

//will return 1. the word that I want and 2. the rest of the string (if present)
fn get_next_word(request: &str) -> Option<(&str, &str)> {

    /*
    too much code
    let mut iter = request.chars();
    loop {
        let item = iter.next();
        match item {
            Some(c) => {}
            None => break,
        }
    }
    */

    for (i, c) in request.chars().enumerate() {

        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1 ..]));
        }

    }

    None

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

impl From<Utf8Error> for ParseError {

    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
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