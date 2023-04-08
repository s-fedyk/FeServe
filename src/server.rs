use std::{env, str};
use std::env::current_dir;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::fs::File;
use crate::http::{Request, Response, StatusCode};
pub struct Server {
    address: String,
}

impl Server {
    // capital Self is an alias for whatever class self is
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
                                Ok(request) => {
                                    let mut response_body = [0; 1024];
                                    match File::open(request.getPath()) {
                                        Ok(mut file) => {
                                            match file.read(&mut response_body) {
                                                Ok(_) => {write!(stream, "{}",Response::new(StatusCode::Ok,Some(&response_body)));}
                                                Err(_) => {write!(stream, "{}",Response::new(StatusCode::BadRequest,None));}
                                            }
                                        },
                                        Err(_) => {
                                            write!(stream, "{}",Response::new(StatusCode::BadRequest,None));
                                        }
                                    }
                                    dbg!(request);
                                },
                                Err(e) =>{}
                            }
                        },
                        Err(e) => {
                            println!("error {}", e);
                        },
                    }
                    println!()
                },
                Err(e) => {
                    // bind error
                    println!("{}", e);
                }
            }
        }
    }
}