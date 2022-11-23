
//import Method enum from http module
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use::std::error::Error;
use::std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use::std::str;
use::std::str::Utf8Error;
use super::QueryString;

//extend debug trait implementation for all struct fields
#[derive(Debug)]
pub struct Request<'buf> {

    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

    fn query_string(&self) -> Option<&QueryString> {
        //as_ref() converts &Option<QueryString> to Option<&QueryString>
        self.query_string.as_ref()
    }
}
//trait implementation
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {

    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n... HEADERS
    fn try_from (buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

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
        let (mut path, request) = get_next_word(request).ok_or(ParseError::Invalidrequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::Invalidrequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self{
            path,
            query_string,
            method,
        })
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

impl From<MethodError> for ParseError {

    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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