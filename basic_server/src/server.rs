//modules are private by default
use::std::net::TcpListener;
use::std::io::Read;

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
                Ok((mut stream, sock_addr)) => {
                    //do something
                    println!("Receiving stream of bytes on socket: {}", sock_addr);
                    //buffer for storing incoming bytes
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {

                            println!("Received a request: {}", String::from_utf8_lossy(&mut buffer));

                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }

                },

                Err(e) => println!("Failed to establish connection: {}", e),

                //default case
                //_ => {handle default case}
            }

            
        }
    }
}
