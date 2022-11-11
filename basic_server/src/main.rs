//silencing compiler warnings
#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    println!("Hello, server!");

    /*
    let string = String::from("127.0.0.1:8080");
    //takes from BYTE 10 to the end
    let slice = &string[10..];

    dbg!(&string);
    dbg!(&slice);
    */

    //let get = Method::GET("abcd".to_string());
    //let delete = Method::DELETE(10);

    //new is an associated function
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}


/*
code before splitting
mod http {
    //submodule
    pub mod request {
    }
    pub mod method{
    
    }
}
*/