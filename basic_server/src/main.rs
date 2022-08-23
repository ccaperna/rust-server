

fn main() {
    println!("Hello, server!");

    /*
    let string = String::from("127.0.0.1:8080");
    //takes from BYTE 10 to the end
    let slice = &string[10..];

    dbg!(&string);
    dbg!(&slice);
    */

    let get = Method::GET("abcd".to_string());
    let delete = Method::DELETE(10);

    //new is an associated function
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {

    fn new(address: String) -> Self {

        Self {
            address
        }
    }

    //this func takes ownership beacuse we don't pass a reference
    fn run(self) {

        println!("Listening on {}", self.address);

    }
}

struct Request {

    path: String,
    query: String,
    method: Method,
}

enum Method {

    GET(String),
    DELETE(i64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
