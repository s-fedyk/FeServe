use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
pub struct Server {
    address: String,
}

impl Server {
    // capital Self is an alias for server (Server)
    pub fn new(address: String) -> Self {
        Server {
            address
        }
    }
    // note, dont take ownership of the struct, so we dont kill our server
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, socket_addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            match Request::try_from(&buffer[..] ) {
                                Ok(request) => {},
                                Err(e) =>{}
                            }
                        },
                        Err(e) => {
                            println!("error {}", e);
                        },
                        _ => {
                            println!("unhandled case")
                        }
                    }
                    println!()
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}