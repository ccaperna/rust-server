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

        let listener = TcpListener::bind(&self.address).unwrap();
    }
}
