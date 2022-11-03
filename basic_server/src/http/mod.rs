
//select needed structs from modules
pub use request::Request;
pub use method::Method;
pub use request::ParseError;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use response::Response;

//select modules
pub mod request;
pub mod method;
pub mod query_string;
pub mod response;