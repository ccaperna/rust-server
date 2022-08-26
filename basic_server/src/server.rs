//modules are private by default
use::std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {

    pub fn new(address: String) -> Self {

        Self {
            address
        }
    }

    //this func takes ownership beacuse we don't pass a reference
    pub fn run(self) {

        println!("Listening on {}", self.address);

        //unwrap will terminate the program in case of error
        let listener = TcpListener::bind(&self.address).unwrap();

        /*
        //loop labeling
        'outer: loop {
            loop {
                //breaks outer loop
                break 'outer;
            }
        }
        */

        loop {
            match listener.accept() {
                Ok((stream, sock_addr)) => {
                    //do something
                    println!("OK");
                },

                Err(e) => println!("Failed to establish connection: {}", e),

                //default case
                //_ => {handle default case}
            }

            
        }
    }
}
