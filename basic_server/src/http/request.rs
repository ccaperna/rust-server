//import Method enum from http module
use super::method::Method;

pub struct Request {

    path: String,
    query: String,
    method: Method,
}

